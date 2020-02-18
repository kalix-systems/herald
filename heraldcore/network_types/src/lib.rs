pub mod action;
pub mod cmessages;
pub mod dmessages;

use herald_common::kson::prelude::*;
use herald_ids::*;

#[derive(Debug, Ser, De)]
pub enum Substance {
    Cm {
        cid: ConversationId,
        msg: cmessages::ConversationMessage,
    },
    Dm(dmessages::DeviceMessage),
}

mod rusqlite_imp;
