//use coremacros::exit_err;
use herald_common::UserId;
use herald_ids::ConversationId;
//use once_cell::sync::OnceCell;
use coremacros::w;
use ratchet_chat::{protocol::ConversationStore, StoreLike};
use rusqlite::named_params as np;

pub struct Conn<'conn>(rusqlite::Transaction<'conn>);

impl<'conn> std::ops::Deref for Conn<'conn> {
    type Target = rusqlite::Transaction<'conn>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! sql {
    ($category:literal, $file:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/sql/",
            $category,
            "/",
            $file,
            ".sql"
        ))
    };
}

macro_rules! prep {
    ($slf: ident, $category:literal, $file:literal) => {
        w!($slf.prepare_cached(sql!($category, $file)))
    };
}

impl StoreLike for Conn<'_> {
    type Error = rusqlite::Error;
}

impl ConversationStore for Conn<'_> {
    fn add_to_convo(
        &mut self,
        cid: ConversationId,
        members: Vec<UserId>,
    ) -> Result<(), Self::Error> {
        let mut stmt = prep!(self, "members", "add");

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
        let mut stmt = prep!(self, "members", "remove");

        w!(stmt.execute_named(np!("@conversation_id": cid, "@user_id": from)));

        Ok(())
    }

    fn get_members(
        &mut self,
        cid: ConversationId,
    ) -> Result<Vec<UserId>, Self::Error> {
        let mut stmt = prep!(self, "members", "get_members");

        let res = stmt.query_map_named(np!("@conversation_id": cid), |row| {
            row.get::<_, UserId>("user_id")
        });

        w!(res).collect()
    }

    fn member_of(
        &mut self,
        cid: ConversationId,
        uid: UserId,
    ) -> Result<bool, Self::Error> {
        let mut stmt = prep!(self, "members", "member_of");

        stmt.query_row_named(np!("@conversation_id": cid, "@user_id": uid), |row| {
            row.get::<_, bool>(0)
        })
    }
}
