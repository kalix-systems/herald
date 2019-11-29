use crate::ffi;
pub use crate::herald::Herald;
use crate::imp::*;
use crate::interface::*;

macro_rules! load_props {
    ($( $field: ident, $mut: ident, $ret: ty),*) => {
       $(
       fn $field(&self) -> &$ret {
            &self.load_props.$field
       }

       fn $mut(&mut self) -> &mut $ret {
            &mut self.load_props.$field
       }
       )*
    }
}

macro_rules! props {
    ($( $field: ident, $mut: ident, $ret: ty),*) => {
       $(
       fn $field(&self) -> &$ret {
            &self.$field
       }

       fn $mut(&mut self) -> &mut $ret {
            &mut self.$field
       }
       )*
    }
}

impl HeraldTrait for Herald {
    fn new(
        emit: HeraldEmitter,
        _: HeraldList,
        config: Config,
        conversation_builder: ConversationBuilder,
        conversations: Conversations,
        errors: Errors,
        message_search: MessageSearch,
        users: Users,
        users_search: UsersSearch,
        utils: Utils,
    ) -> Self {
        Self::new_(
            emit,
            config,
            conversation_builder,
            conversations,
            errors,
            message_search,
            users,
            users_search,
            utils,
        )
    }

    fn config_init(&self) -> bool {
        self.config_init_()
    }

    fn register_new_user(
        &mut self,
        user_id: ffi::UserId,
    ) {
        self.register_new_user_(user_id)
    }

    fn can_fetch_more(&self) -> bool {
        self.can_fetch_more_()
    }

    fn fetch_more(&mut self) {
        self.fetch_more_()
    }

    fn login(&mut self) -> bool {
        self.login_()
    }

    fn connection_up(&self) -> bool {
        self.connection_up_()
    }

    fn connection_pending(&self) -> bool {
        self.connection_pending_()
    }

    fn emit(&mut self) -> &mut HeraldEmitter {
        self.emit_()
    }

    load_props! {
        config, config_mut, Config,
        conversation_builder, conversation_builder_mut, ConversationBuilder,
        conversations, conversations_mut, Conversations,
        users, users_mut, Users
    }

    props! {
        errors, errors_mut, Errors,
        message_search, message_search_mut, MessageSearch,
        users_search, users_search_mut, UsersSearch,
        utils, utils_mut, Utils
    }

    fn row_count(&self) -> usize {
        0
    }
}
