use super::*;
use crate::ret_none;

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
        }
    }

    pub(crate) fn config_init_(&self) -> bool {
        self.load_props.config.loaded()
    }

    pub(crate) fn register_new_user_(
        &mut self,
        user_id: ffi::UserId,
    ) {
        use register::*;

        let uid = ret_err!(UserId::try_from(user_id.as_str()));

        let mut emit = self.emit.clone();

        spawn!(match ret_err!(net::register(uid)) {
            Res::UIDTaken => {
                eprintln!("UID taken!");
            }
            Res::KeyTaken => {
                eprintln!("Key taken!");
            }
            Res::BadSig(s) => {
                eprintln!("Bad sig: {:?}", s);
            }
            Res::Success => {
                ret_err!(push(shared::Update::RegistrationSuccess));
                emit.new_data_ready();
            }
        });
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
            ret_err!(net::login(
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
        ret_none!(heraldcore::set_data_dir(std::path::PathBuf::from(
            ret_none!(crate::utils::strip_qrc(path))
        )));

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
