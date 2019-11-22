use super::*;
use channel_ratchet::*;

fn in_memory() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open_in_memory().expect(womp!());
    conn.execute_batch(include_str!("sql/create.sql"))
        .expect(womp!());
    conn
}

#[test]
fn ratchet_states() {
    let mut conn = in_memory();
    let cid1 = ConversationId::from([1; 32]);
    let cid2 = ConversationId::from([2; 32]);

    let r1 = RatchetState::new();
    let r2 = RatchetState::new();

    assert_ne!(r1, r2);

    let res: Result<(), ChainKeysError> = db::with_tx_from_conn(&mut conn, |tx| {
        tx.store_ratchet_state(cid1, &r1)?;
        tx.store_ratchet_state(cid2, &r2)?;
        Ok(())
    });

    res.expect(womp!());

    let res: Result<_, ChainKeysError> = db::with_tx_from_conn(&mut conn, |tx| {
        Ok((tx.get_ratchet_state(cid1)?, tx.get_ratchet_state(cid2)?))
    });

    let (r1_, r2_) = res.expect(womp!());

    assert_eq!(r1, r1_);
    assert_eq!(r2, r2_);

    let ad1 = Bytes::from_static(b"wat");
    let ad2 = Bytes::from_static(b"asdf");

    let msg1 = BytesMut::from(b"asdf" as &[u8]);
    let msg2 = BytesMut::from(b"wat" as &[u8]);

    assert_ne!(msg1, msg2);

    let res: Result<_, ChainKeysError> = db::with_tx_from_conn(&mut conn, |tx| {
        let c1 = tx.seal_msg(cid1, ad1.clone(), msg1)?;
        let c2 = tx.seal_msg(cid2, ad2.clone(), msg2)?;
        Ok((c1, c2))
    });
    let (c1, c2) = res.expect(womp!());

    let res: Result<_, ChainKeysError> = db::with_tx_from_conn(&mut conn, |tx| {
        let d1 = tx.open_msg(cid1, c1)?;
        let d2 = tx.open_msg(cid2, c2)?;
        Ok((d1, d2))
    });
    let (d1, d2) = res.expect(womp!());

    let d1 = d1.expect(womp!());
    let d2 = d2.expect(womp!());

    assert_eq!(d1.ad, ad1);
    assert_eq!(d2.ad, ad2);

    assert_eq!(&d1.pt, b"asdf" as &[u8]);
    assert_eq!(&d2.pt, b"wat" as &[u8]);
}
