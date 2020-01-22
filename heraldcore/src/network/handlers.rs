use super::*;

macro_rules! e {
    ($val: expr) => {
        match $val {
            Ok(_val) => _val,
            Err(e) => {
                $crate::err(e);
                return;
            }
        }
    };
}

pub(super) fn handle_push(
    Push {
        timestamp,
        msg,
        gid,
        ..
    }: Push
) {
    use crypto_store::prelude::{Msg as PLMsg, *};

    let payload: PLMsg = e!(kson::from_bytes(msg));
    let kp = e!(crate::config::keypair());
    let id = e!(crate::config::id());

    let mut raw = raw_conn().lock();
    let mut tx: Conn = e!(raw.transaction()).into();

    let MsgResult {
        ack,
        forward,
        output,
        response,
    } = e!(handle_incoming(&mut tx, &kp, gid, payload));

    if let Some(ack) = ack {
        todo!();
    }

    if let Some(forward) = forward {
        let bytes = kson::to_bytes(&forward);
        prepare_send_to_self(&mut tx, &kp, id, Payload::Msg(bytes));
        todo!();
    }

    if let Some(output) = output {
        let msg: ConversationMessage = e!(kson::from_bytes(output));
        todo!();
    }

    if let Some(response) = response {
        todo!();
    }

    e!(tx.commit());
}
