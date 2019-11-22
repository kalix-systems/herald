use super::*;
pub use coretypes::attachments::*;

pub(crate) mod db;

/// Gets all attachments associated with a message id
pub fn get(msg_id: &MsgId) -> Result<AttachmentMeta, HErr> {
    let db = Database::get()?;
    db::get(&db, msg_id)
}

#[cfg(test)]
mod tests;
