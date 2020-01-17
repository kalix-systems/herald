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

        w!(res).collect()
    }

    fn member_of(
        &mut self,
        cid: ConversationId,
        uid: UserId,
    ) -> Result<bool, Self::Error> {
        let mut stmt = st!(self, "members", "member_of");
        let params = np!("@conversation_id": cid, "@user_id": uid);

        stmt.query_row_named(params, |row| row.get(0))
    }
}
