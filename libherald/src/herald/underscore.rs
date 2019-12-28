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
        message_search: MessageSearch,
        users: Users,
        users_search: UsersSearch,
        utils: Utils,
    ) -> Self {
        let global_emit = emit.clone();
        shared::set_emitter(global_emit);

        Herald {
            emit,
            effects_flags: Arc::new(EffectsFlags::new()),
            message_search,
            load_props: imp::LoadProps {
                config,
                conversation_builder,
                conversations,
                users,
            },
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
        use register::*;

        let addr = if !(server_addr.is_empty() && server_port.is_empty()) {
            Some(err!(format!("{}:{}", server_addr, server_port).parse()))
        } else {
            None
        };

        let uid = err!(UserId::try_from(user_id.as_str()));

        spawn!(
            match push_err!(net::register(uid, addr), "Registration failed") {
                Some(Res::UIDTaken) => {
                    push(shared::RegistrationFailureCode::UserIdTaken);
                }
                Some(Res::KeyTaken) => {
                    push(shared::RegistrationFailureCode::KeyTaken);
                }
                Some(Res::BadSig(_)) => {
                    push(shared::RegistrationFailureCode::BadSignature);
                }
                Some(Res::Success) => {
                    push(shared::Update::RegistrationSuccess);
                }
                None => {
                    push(shared::RegistrationFailureCode::Other);
                }
            }
        );
    }

    pub(crate) fn can_fetch_more_(&self) -> bool {
        shared::more_updates()
    }

    pub(crate) fn fetch_more_(&mut self) {
        self.process_updates()
    }

    pub(crate) fn connection_up_(&self) -> bool {
        self.effects_flags.net_online.load(Ordering::Relaxed)
    }

    pub(crate) fn connection_pending_(&self) -> bool {
        self.effects_flags.net_pending.load(Ordering::Relaxed)
    }

    pub(crate) fn login_(&mut self) -> bool {
        use heraldcore::errors::HErr;

        let mut handler = NotifHandler::new(self.emit.clone(), self.effects_flags.clone());

        spawn!(
            err!(net::login(
                move |notif: Notification| {
                    handler.send(notif);
                },
                move |herr: HErr| {
                    push_err!(Err::<(), HErr>(herr), "Problem in login thread");
                }
            )),
            false
        );

        true
    }

    pub(crate) fn emit_(&mut self) -> &mut HeraldEmitter {
        &mut self.emit
    }

    pub(crate) fn set_app_local_data_dir_(
        &mut self,
        path: String,
    ) {
        if let Some(path) = crate::utils::strip_qrc(path) {
            none!(heraldcore::set_data_dir(std::path::PathBuf::from(path)));
        }

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
