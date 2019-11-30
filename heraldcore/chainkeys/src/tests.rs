use super::*;
use coremacros::womp;
use kdf_ratchet::*;

fn in_memory() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open_in_memory().expect(womp!());
    conn.execute_batch(include_str!("sql/create.sql"))
        .expect(womp!());
    conn
}

#[test]
fn ratchet_states() {
    kcl::init();

    let mut conn = in_memory();
    // // use a real db for debugging failures
    // let mut conn = CK_CONN.lock();
    // conn.execute_batch(include_str!("sql/create.sql"))
    //     .expect(womp!());

    let cid1 = ConversationId::from([1; 32]);
    let cid2 = ConversationId::from([2; 32]);

    assert_ne!(cid1, cid2);

    let pk1 = *sig::KeyPair::gen_new().public_key();
    let pk2 = *sig::KeyPair::gen_new().public_key();

    assert_ne!(pk1, pk2);

    let r1 = RatchetState::gen_new();
    let r2 = RatchetState::gen_new();
    let r3 = RatchetState::gen_new();
    let r4 = RatchetState::gen_new();

    assert_ne!(r1, r2);
    assert_ne!(r1, r3);
    assert_ne!(r1, r4);

    assert_ne!(r2, r1);
    assert_ne!(r2, r3);
    assert_ne!(r2, r4);

    assert_ne!(r3, r1);
    assert_ne!(r3, r2);
    assert_ne!(r3, r4);

    assert_ne!(r4, r1);
    assert_ne!(r4, r2);
    assert_ne!(r4, r3);

    let res: Result<(), ChainKeysError> = db::with_tx_from_conn(&mut conn, |tx| {
        tx.store_ratchet_state(cid1, pk1, 0, &r1)?;
        tx.store_ratchet_state(cid2, pk1, 0, &r2)?;

        tx.store_ratchet_state(cid1, pk2, 0, &r3)?;
        tx.store_ratchet_state(cid2, pk2, 0, &r4)?;
        Ok(())
    });

    res.expect(womp!());

    let res: Result<_, ChainKeysError> = db::with_tx_from_conn(&mut conn, |tx| {
        Ok((
            tx.get_recent_ratchet(cid1, pk1)?,
            tx.get_recent_ratchet(cid2, pk1)?,
            tx.get_recent_ratchet(cid1, pk2)?,
            tx.get_recent_ratchet(cid2, pk2)?,
        ))
    });

    let ((gr1, r1_), (gr2, r2_), (gr3, r3_), (gr4, r4_)) = res.expect(womp!());

    assert_eq!(gr1, 0);
    assert_eq!(gr2, 0);
    assert_eq!(gr3, 0);
    assert_eq!(gr4, 0);

    assert_eq!(r1, r1_);
    assert_eq!(r2, r2_);
    assert_eq!(r3, r3_);
    assert_eq!(r4, r4_);

    let ad1 = Bytes::from_static(b"wat");
    let ad2 = Bytes::from_static(b"asdf");
    let ad3 = Bytes::from_static(b"123");
    let ad4 = Bytes::from_static(b"watata");

    let msg1 = BytesMut::from(b"asdf" as &[u8]);
    let msg2 = BytesMut::from(b"wat" as &[u8]);
    let msg3 = BytesMut::from(b"watata" as &[u8]);
    let msg4 = BytesMut::from(b"123" as &[u8]);

    let res: Result<_, ChainKeysError> = db::with_tx_from_conn(&mut conn, |tx| {
        let c1 = tx.seal_msg(cid1, pk1, ad1.clone(), msg1.clone())?;
        let c2 = tx.seal_msg(cid2, pk1, ad2.clone(), msg2.clone())?;
        let c3 = tx.seal_msg(cid1, pk2, ad3.clone(), msg3.clone())?;
        let c4 = tx.seal_msg(cid2, pk2, ad4.clone(), msg4.clone())?;
        Ok((c1, c2, c3, c4))
    });
    let ((gc1, c1), (gc2, c2), (gc3, c3), (gc4, c4)) = res.expect(womp!());

    assert_eq!(gc1, 0);
    assert_eq!(gc2, 0);
    assert_eq!(gc3, 0);
    assert_eq!(gc4, 0);

    let res: Result<_, ChainKeysError> = db::with_tx_from_conn(&mut conn, |tx| {
        let m1 = tx.get_derived_key(cid1, pk1, 0, 0)?;
        let m2 = tx.get_derived_key(cid2, pk1, 0, 0)?;
        let m3 = tx.get_derived_key(cid1, pk2, 0, 0)?;
        let m4 = tx.get_derived_key(cid2, pk2, 0, 0)?;
        Ok((m1, m2, m3, m4))
    });

    let (m1, m2, m3, m4) = res.expect(womp!());

    drop(m1.expect(womp!()));
    drop(m2.expect(womp!()));
    drop(m3.expect(womp!()));
    drop(m4.expect(womp!()));

    let res: Result<_, ChainKeysError> = db::with_tx_from_conn(&mut conn, |tx| {
        let d1 = tx.open_msg(cid1, pk1, 0, c1)?;
        let d2 = tx.open_msg(cid2, pk1, 0, c2)?;
        let d3 = tx.open_msg(cid1, pk2, 0, c3)?;
        let d4 = tx.open_msg(cid2, pk2, 0, c4)?;
        Ok((d1, d2, d3, d4))
    });
    let (d1, d2, d3, d4) = res.expect(womp!());

    let d1 = d1.expect(womp!());
    let d2 = d2.expect(womp!());
    let d3 = d3.expect(womp!());
    let d4 = d4.expect(womp!());

    assert_eq!(d1.ad, ad1);
    assert_eq!(d2.ad, ad2);
    assert_eq!(d3.ad, ad3);
    assert_eq!(d4.ad, ad4);

    assert_eq!(d1.pt, msg1);
    assert_eq!(d2.pt, msg2);
    assert_eq!(d3.pt, msg3);
    assert_eq!(d4.pt, msg4);
}
