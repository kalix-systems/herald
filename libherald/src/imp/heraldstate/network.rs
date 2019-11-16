use super::*;

/// A bundle of `AtomicBool`s used for signalling
pub struct EffectsFlags {
    pub(super) net_online: AtomicBool,
    pub(super) net_pending: AtomicBool,
}

/// This is passed inside of a callback to the login function,
/// and sends signals and notifications to the QML runtime.
pub struct NotifHandler {
    pub(super) _effects_flags: Arc<EffectsFlags>,
    pub(super) _emit: Emitter,
}

impl NotifHandler {
    pub(super) fn send(&mut self, notif: Notification) {
        use crate::imp::conversations::shared::*;
        use crate::imp::users::{shared::*, Users};
        use messages::{shared::MsgUpdate, Messages};
        use Notification::*;

        match notif {
            NewMsg(msg_id, cid) => {
                ret_err!(Messages::push(cid, MsgUpdate::Msg(msg_id)));
            }
            MsgReceipt { mid, cid } => {
                ret_err!(Messages::push(cid, MsgUpdate::Receipt(mid)));
            }
            NewUser(uid, cid) => {
                // add user
                ret_err!(Users::push(UsersUpdates::NewUser(uid)));

                // add pairwise conversation
                ret_err!(Conversations::push(ConvUpdate::NewConversation(cid)));
            }
            NewConversation(cid) => {
                ret_err!(Conversations::push(ConvUpdate::NewConversation(cid)));
            }
            AddUserResponse(cid, uid, accepted) => {
                // handle response
                ret_err!(Users::push(UsersUpdates::ReqResp(uid, accepted)));

                // add conversation
                if accepted {
                    ret_err!(Conversations::push(ConvUpdate::NewConversation(cid)));
                }
            }
            AddConversationResponse(cid, uid, accepted) => {
                use crate::imp::members::{shared::*, Members};
                ret_err!(Members::push(cid, MemberUpdate::ReqResp(uid, accepted)));
            }
            Settings(cid, settings) => {
                ret_err!(Conversations::push(ConvUpdate::Settings(cid, settings)));
            }
        }
    }
    pub(super) fn new(emit: Emitter, _effects_flags: Arc<EffectsFlags>) -> Self {
        Self {
            _effects_flags,
            _emit: emit,
        }
    }
}

impl EffectsFlags {
    /// Creates a new `EffectsFlags`
    pub fn new() -> Self {
        EffectsFlags {
            net_online: AtomicBool::new(false),
            net_pending: AtomicBool::new(false),
        }
    }
}
