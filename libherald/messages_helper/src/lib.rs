use heraldcore::message::MsgData;

pub mod container;
pub mod search;
pub mod types;

pub fn media_attachments_json(
    attachments: &heraldcore::message::attachments::AttachmentMeta
) -> Option<String> {
    if attachments.is_empty() {
        return None;
    }

    let media = attachments.media_attachments().ok()?;

    if media.is_empty() {
        return None;
    }

    Some(json::JsonValue::from(media).dump())
}

pub fn doc_attachments_json(
    attachments: &heraldcore::message::attachments::AttachmentMeta
) -> Option<String> {
    if attachments.is_empty() {
        return None;
    }

    let docs = attachments.doc_attachments().ok()?;

    if docs.is_empty() {
        return None;
    }

    Some(json::JsonValue::from(docs).dump())
}
