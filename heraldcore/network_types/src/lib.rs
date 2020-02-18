pub mod action;
pub mod cmessages;
pub mod umessages;

use herald_common::kson::prelude::*;
use herald_ids::*;

#[derive(Debug, Ser, De)]
pub enum Substance {
    Cm {
        cid: ConversationId,
        msg: cmessages::ConversationMessage,
    },
    Um(umessages::UserMessage),
}

mod rusqlite_imp;
