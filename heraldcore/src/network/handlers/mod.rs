use super::*;
use coremacros::w;
use crypto_store::prelude::*;
use network_types as nt;

mod substance;

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

#[derive(Default)]
pub(super) struct Pushy;

impl PushHandler for Pushy {
    fn handle(
        &mut self,
        Push {
            timestamp,
            msg,
            gid,
            ..
        }: Push,
    ) {
        let mut ev = Event::default();

        let payload: Msg = e!(kson::from_bytes(msg));
        let kp = e!(crate::config::keypair());
        let id = e!(crate::config::id());

        let mut raw = raw_conn().lock();
        let mut conn: Conn = e!(raw.transaction()).into();
        let tx = &mut conn;

        let MsgResult {
            ack,
            forward,
            output,
            response,
        } = e!(handle_incoming(tx, &kp, gid, payload));

        if let Some(ack) = ack {
            ev.reply(gid.did, Msg::Ack(ack));
        }

        if let Some(forward) = forward {
            let bytes = kson::to_bytes(&forward);
            let replies = e!(prepare_send_to_self(tx, &kp, id, Payload::Msg(bytes)));

            ev.replies(replies);
        }

        if let Some(response) = response {
            ev.reply(gid.did, response);
        }

        if let Some(output) = output {
            if let Ok(output_ev) = handle_output(output, gid.uid, timestamp) {
                ev.merge(output_ev);
            }
        }

        e!(conn.commit());
        e!(ev.execute());
    }
}

fn handle_output(
    output: PayloadResult,
    from: UserId,
    ts: Time,
) -> Result<Event, HErr> {
    use PayloadResult as P;
    let mut ev = Event::default();

    match output {
        P::Msg(bytes) => {
            let msg: nt::NetMsg = w!(kson::from_bytes(bytes));
        }

        P::AddToConvo(cid, members) => {
            w!(members::add_members(&cid, &members));

            ev.note(members::Membership {
                cid,
                change: members::MembershipUpdate::Added {
                    members,
                    added_by: from,
                },
            });
        }

        P::LeaveConvo(cid) => {
            w!(members::remove_member(&cid, from));

            ev.note(members::Membership {
                cid,
                change: members::MembershipUpdate::Left(from),
            });
        }
    };

    Ok(ev)
}
