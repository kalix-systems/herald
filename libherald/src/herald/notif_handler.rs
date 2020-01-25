use super::*;
use crate::{content_push, messages};

/// This is passed inside of a callback to the register handler function,
/// and sends signals and notifications to the QML runtime.
pub struct NotifHandler;

impl NotifHandler {
    pub(super) fn send(
        &mut self,
        notif: Notification,
    ) {
        use crate::conversations::shared::*;
        use crate::members::MemberUpdate;
        use crate::users::shared::*;
        use heraldcore::message;
        use messages::MsgUpdate;
        use Notification::*;

        match notif {
            NewMsg(msg) => {
                crate::toasts::new_msg_toast(msg.as_ref());
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
            TypingIndicator(cid, uid) => {
                err!(content_push(cid, MemberUpdate::TypingIndicator(uid)));
            }
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
                err!(content_push(cid, MemberUpdate::ReqResp(uid, accepted)));
            }
            Settings(cid, settings) => {
                err!(content_push(cid, settings));
            }
            GC(convs) => convs.into_iter().for_each(|(cid, mids)| {
                err!(content_push(cid, MsgUpdate::ExpiredMessages(mids)));
            }),
            OutboundMsg(update) => {
                use heraldcore::message::StoreAndSend::*;

                match update {
                    Msg(cid, msg) => {
                        err!(content_push(cid, MsgUpdate::BuilderMsg(msg)));
                    }
                    StoreDone(cid, mid, meta) => {
                        err!(content_push(cid, MsgUpdate::StoreDone(mid, meta)));
                    }
                    SendDone(cid, mid) => {
                        err!(content_push(cid, MsgUpdate::SendDone(mid)));
                    }
                }
            }
            OutboundAux(update) => {
                use heraldcore::message::OutboundAux::*;
                match update {
                    Msg(msg) => {
                        err!(content_push(msg.conversation, MsgUpdate::BuilderMsg(msg)));
                    }
                    SendDone(cid, mid) => {
                        err!(content_push(cid, MsgUpdate::SendDone(mid)));
                    }
                }
            }
            UserChanged(uid, update) => {
                push(UserUpdate::UserChanged(uid, update));
            }
        }
    }

    pub(super) fn new() -> Self {
        Self
    }
}
