use super::*;

pub(in crate::messages) fn handle_receipt(
    container: &mut Container,
    mid: MsgId,
    status: MessageReceiptStatus,
    recipient: UserId,
    model: &mut List,
) -> Result<(), HErr> {
    let res = update(&mid, |msg| {
        msg.receipts
            .entry(recipient)
            .and_modify(|v| {
                if *v < status {
                    *v = status
                }
            })
            .or_insert(status);
    });

    if res.is_none() {
        return Ok(());
    }

    // NOTE: If this fails, there is a bug somewhere
    // in libherald.
    //
    // It is probably trivial, but may reflect something
    // deeply wrong with our understanding of the program's
    // concurrency.
    let ix = container
        .list
        .iter()
        // search backwards,
        // it's probably fairly recent
        .rposition(|m| m.msg_id == mid)
        .ok_or(NE!())?;

    model.data_changed(ix, ix);

    Ok(())
}

pub(in crate::messages) fn handle_store_done(
    container: &mut Container,
    mid: MsgId,
    meta: heraldcore::message::attachments::AttachmentMeta,
    model: &mut List,
) -> Option<()> {
    update(&mid, move |data| {
        data.attachments = meta;
    })?;

    let ix = container
        .list
        .iter()
        // search backwards,
        // it's probably very recent
        .rposition(|m| m.msg_id == mid)?;

    model.data_changed(ix, ix);

    Some(())
}
