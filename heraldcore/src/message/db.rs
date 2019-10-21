use super::*;
use std::ops::{Deref, DerefMut};

pub(super) fn store_and_send<F: FnMut(StoreAndSend) + Send + 'static, D>(
    mut db: D,
    builder: OutboundMessageBuilder,
    mut callback: F,
) -> Result<(), HErr>
where
    D: Deref<Target = Database> + DerefMut + Send + 'static,
{
    let OutboundMessageBuilder {
        conversation,
        mut body,
        op,
        attachments,
        parse_markdown,
    } = builder;

    use MissingOutboundMessageField::*;

    if parse_markdown {
        body = match body {
            Some(body) => Some(body.parse_markdown()?),
            None => None,
        };
    }

    if attachments.is_empty() && body.is_none() {
        return Err(MissingBody.into());
    }

    let conversation_id = conversation.ok_or(MissingConversationId)?;
    let msg_id: MsgId = utils::rand_id().into();
    let timestamp = Time::now();
    let author = crate::config::Config::static_id()?;
    let send_status = MessageSendStatus::NoAck;

    let receipts: HashMap<UserId, MessageReceiptStatus> = HashMap::default();
    let receipts_bytes = serde_cbor::to_vec(&receipts)?;
    let has_attachments = !attachments.is_empty();

    let msg = Message {
        message_id: msg_id,
        author,
        body: (&body).clone(),
        op,
        conversation: conversation_id,
        timestamp,
        send_status,
        receipts,
        has_attachments,
    };

    callback(StoreAndSend::Msg(msg));

    macro_rules! e {
        ($res: expr) => {
            match $res {
                Ok(val) => val,
                Err(e) => {
                    callback(StoreAndSend::Error {
                        error: e.into(),
                        line_number: line!(),
                    });
                    return;
                }
            }
        };
    }

    std::thread::Builder::new().spawn(move || {
        let attachments: Result<Vec<Attachment>, HErr> = attachments
            .into_iter()
            .map(|path| {
                let attach: Attachment = Attachment::new(&path)?;

                attach.save()?;

                Ok(attach)
            })
            .collect();
        let attachments = e!(attachments);

        let tx = e!(db.transaction());

        e!(tx.execute(
            include_str!("sql/add.sql"),
            params![
                msg_id,
                author,
                conversation_id,
                body,
                send_status,
                receipts_bytes,
                has_attachments,
                timestamp,
            ],
        ));

        e!(tx.execute(
            include_str!("../conversation/sql/update_last_active.sql"),
            params![timestamp, conversation_id],
        ));

        if let Some(op) = op {
            e!(tx.execute(include_str!("sql/add_reply.sql"), params![msg_id, op]));
        }

        if !attachments.is_empty() {
            e!(attachments::add_db(
                &tx,
                &msg_id,
                attachments.iter().map(|a| a.hash_dir())
            ));
        }

        e!(tx.commit());

        callback(StoreAndSend::StoreDone(msg_id));

        let content = cmessages::Message { body, attachments };
        let msg = cmessages::Msg {
            mid: msg_id,
            content,
            op,
        };
        e!(crate::network::send_normal_message(conversation_id, msg));

        callback(StoreAndSend::SendDone(msg_id));
    })?;

    Ok(())
}
