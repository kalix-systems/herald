use super::*;
use crate::imp::messages;

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
        use crate::imp::{conversations::shared::*, users::shared::*};
        use heraldcore::message;
        use messages::{shared::MsgUpdate, Messages};
        use Notification::*;

        match notif {
            NewMsg(msg) => {
                let cid = msg.conversation;
                ret_err!(Messages::push(cid, MsgUpdate::NewMsg(msg)));
            }
            MsgReceipt(message::MessageReceipt {
                msg_id,
                cid,
                recipient,
                status,
            }) => {
                ret_err!(Messages::push(
                    cid,
                    MsgUpdate::Receipt {
                        msg_id,
                        recipient,
                        status
                    }
                ));
            }
            NewUser(update) => {
                let (user, meta) = *update;
                // add user
                ret_err!(Users::push(UsersUpdates::NewUser(user)));

                // add pairwise conversation
                ret_err!(Conversations::push(ConvUpdate::NewConversation(meta)));
            }
            NewConversation(meta) => {
                ret_err!(Conversations::push(ConvUpdate::NewConversation(meta)));
            }
            AddUserResponse(cid, uid, accepted) => {
                // handle response
                ret_err!(Users::push(UsersUpdates::ReqResp(uid, accepted)));

                // add conversation
                if accepted {
                    let meta = ret_err!(heraldcore::conversation::meta(&cid));
                    ret_err!(Conversations::push(ConvUpdate::NewConversation(meta)));
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

impl Herald {
    pub(super) fn login_(&mut self) -> bool {
        use heraldcore::errors::HErr;

        let mut handler = NotifHandler::new(self.emit.clone(), self.effects_flags.clone());

        spawn!(
            ret_err!(net::login(
                move |notif: Notification| {
                    handler.send(notif);
                },
                move |herr: HErr| {
                    ret_err!(Err::<(), HErr>(herr));
                }
            )),
            false
        );
        true
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
