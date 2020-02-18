pub mod action;
pub mod cmessages;
pub mod dmessages;

use herald_common::kson::prelude::*;

#[derive(Debug, Ser, De)]
pub enum Substance {
    Cm(cmessages::ConversationMessage),
    Dm(dmessages::DeviceMessage),
}

mod rusqlite_imp;
