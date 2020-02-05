pub mod action;
pub mod cmessages;
pub mod dmessages;

pub enum Substance {
    Cm(cmessages::ConversationMessage),
    Dm(dmessages::DeviceMessage),
}

mod rusqlite_imp;
