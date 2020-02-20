use super::*;
use crate::none;

impl Herald {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_(
        mut emit: HeraldEmitter,
        config: Config,
        conversation_builder: ConversationBuilder,
        conversations: Conversations,
        errors: Errors,
        notifications: Notifications,
        message_search: MessageSearch,
        users: Users,
        users_search: UsersSearch,
        utils: Utils,
    ) -> Self {
        shared::set_emitter(emit.clone());

        let mut handler = notif_handler::NotifHandler::new();

        push_err!(
            heraldcore::updates::register_handlers(
                move |notif: Notification| {
                    handler.send(notif);
                },
                move |err: heraldcore::errors::HErr| {
                    err!(Err::<(), _>(err));
                },
            ),
            "Failed to register event handlers"
        );

        Herald {
            emit,
            message_search,
            load_props: imp::LoadProps {
                config,
                conversation_builder,
                conversations,
                users,
            },
            notifications,
            errors,
            users_search,
            utils,
            registration_failure_code: None,
        }
    }

    pub(crate) fn config_init_(&self) -> bool {
        self.load_props.config.loaded()
    }

    pub(crate) fn register_new_user_(
        &mut self,
        user_id: ffi::UserId,
        server_addr: String,
        server_port: String,
    ) {
        use protocol::auth::*;

        let addr = if !(server_addr.is_empty() && server_port.is_empty()) {
            Some(err!(format!("{}:{}", server_addr, server_port).parse()))
        } else {
            None
        };

        let uid = err!(UserId::try_from(user_id.as_str()));

        spawn!(
            match push_err!(net::register(uid, addr), "Registration failed") {
                Some(RegisterResponse::Taken) => {
                    push(shared::RegistrationFailureCode::UserIdTaken);
                }
                Some(RegisterResponse::BadSig(_)) => {
                    push(shared::RegistrationFailureCode::BadSignature);
                }
                Some(RegisterResponse::Success) => {
                    push(shared::Update::RegistrationSuccess);
                }
                None => {
                    push(shared::RegistrationFailureCode::Other);
                }
            }
        );
    }

    // TODO these need to come back
    pub(crate) fn connection_up_(&self) -> bool {
        false
    }

    pub(crate) fn connection_pending_(&self) -> bool {
        false
    }

    pub(crate) fn login_(&mut self) -> bool {
        spawn!(err!(net::login()), false);

        true
    }

    pub(crate) fn emit_(&mut self) -> &mut HeraldEmitter {
        &mut self.emit
    }

    pub(crate) fn set_app_local_data_dir_(
        &mut self,
        path: String,
    ) {
        none!(heraldcore::set_data_dir(std::path::PathBuf::from(path)));

        if config::id().is_ok() {
            self.load_props.setup();
            self.emit.config_init_changed();
        } else {
            // If this fails, the file system is in a very bad place.
            // This probably cannot be recovered from, and there's not meaningful
            // sense in which the application can work. But crashing is still a bad look.
            push_err!(db::init(), "Couldn't initialize storage");
        };
    }
}
