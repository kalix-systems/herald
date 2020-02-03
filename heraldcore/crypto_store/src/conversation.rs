use crate::*;
use coremacros::w;
use herald_common::UserId;
use herald_ids::ConversationId;
use ratchet_chat::protocol::ConversationStore;

impl ConversationStore for Conn<'_> {
    fn add_to_convo(
        &mut self,
        cid: ConversationId,
        members: Vec<UserId>,
    ) -> Result<(), Self::Error> {
        let mut stmt = st!(self, "members", "add");

        for member in members {
            w!(stmt.execute_named(np!("@conversation_id": cid, "@user_id": member)));
        }

        Ok(())
    }

    fn left_convo(
        &mut self,
        cid: ConversationId,
        from: UserId,
    ) -> Result<(), Self::Error> {
        w!(st!(self, "members", "remove")
            .execute_named(np!("@conversation_id": cid, "@user_id": from)));

        Ok(())
    }

    fn get_members(
        &mut self,
        cid: ConversationId,
    ) -> Result<Vec<UserId>, Self::Error> {
        let mut stmt = st!(self, "members", "get_members");
        let params = np!("@conversation_id": cid);

        let res = stmt.query_map_named(params, |row| row.get("user_id"));

        w!(res).map(|r| Ok(w!(r))).collect()
    }

    fn member_of(
        &mut self,
        cid: ConversationId,
        uid: UserId,
    ) -> Result<bool, Self::Error> {
        let mut stmt = st!(self, "members", "member_of");
        let params = np!("@conversation_id": cid, "@user_id": uid);

        Ok(w!(stmt.query_row_named(params, |row| row.get(0))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::in_memory;
    use coremacros::womp;
    use std::convert::TryInto;

    #[test]
    fn member_ops() {
        let mut conn = in_memory();
        let mut conn = Conn::from(conn.transaction().expect(womp!()));

        let cid = ConversationId::gen_new();

        // trivial add
        let members = vec![];
        conn.add_to_convo(cid, members).expect(womp!());

        // add ten
        let mut members: Vec<UserId> = (0..10)
            .map(|i| format!("{}", i).as_str().try_into().expect(womp!()))
            .collect();

        conn.add_to_convo(cid, members.clone()).expect(womp!());

        // no error on redundant insert
        conn.add_to_convo(cid, members.clone()).expect(womp!());

        assert_eq!(conn.get_members(cid).expect(womp!()), members);

        let last = members.last().copied().unwrap();

        // last member is in the conversation
        assert!(conn.member_of(cid, last).expect(womp!()));

        // remove member
        conn.left_convo(cid, last).expect(womp!());

        // last member is no longer in the conversation
        assert!(!conn.member_of(cid, last).expect(womp!()));

        members.pop();
        assert_eq!(conn.get_members(cid).expect(womp!()), members);
    }
}
