use super::*;
use crate::{content_push, messages};
use std::sync::{atomic::AtomicBool, Arc};

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
    pub(super) fn send(
        &mut self,
        notif: Notification,
    ) {
        use crate::conversations::shared::*;
        use crate::users::shared::*;
        use heraldcore::message;
        use messages::MsgUpdate;
        use Notification::*;

        match notif {
            NewMsg(msg) => {
                let cid = msg.conversation;
                err!(content_push(cid, MsgUpdate::NewMsg(msg)));
            }
            MsgReceipt(message::MessageReceipt {
                msg_id,
                cid,
                recipient,
                status,
            }) => {
                err!(content_push(
                    cid,
                    MsgUpdate::Receipt {
                        msg_id,
                        recipient,
                        status
                    }
                ));
            }
            Reaction {
                msg_id,
                reactionary,
                content,
                cid,
                remove,
            } => err!(content_push(
                cid,
                MsgUpdate::Reaction {
                    msg_id,
                    reactionary,
                    content,
                    remove
                }
            )),
            NewUser(update) => {
                let (user, meta) = *update;
                // add user
                push(UserUpdate::NewUser(user));

                // add pairwise conversation
                push(ConvUpdate::NewConversation(meta));
            }
            NewConversation(meta) => {
                push(ConvUpdate::NewConversation(meta));
            }
            AddUserResponse(cid, uid, accepted) => {
                // handle response
                push(UserUpdate::ReqResp(uid, accepted));

                // add conversation
                if accepted {
                    let meta = err!(heraldcore::conversation::meta(&cid));

                    push(ConvUpdate::NewConversation(meta));
                }
            }
            AddConversationResponse(cid, uid, accepted) => {
                use crate::members::MemberUpdate;
                err!(content_push(cid, MemberUpdate::ReqResp(uid, accepted)));
            }
            Settings(cid, settings) => {
                push(ConvUpdate::Settings(cid, settings));
            }
        }
    }
    pub(super) fn new(
        emit: Emitter,
        _effects_flags: Arc<EffectsFlags>,
    ) -> Self {
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
