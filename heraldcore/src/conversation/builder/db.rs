use super::*;
use crate::w;
use rusqlite::named_params;

impl ConversationBuilder {
    fn raw_add(
        self,
        tx: &rusqlite::Transaction,
    ) -> Result<Conversation, HErr> {
        let Self {
            conversation_id,
            title,
            mut picture,
            color,
            muted,
            pairwise,
            expiration_period,
            members,
            status,
            ..
        } = self;

        let id = match conversation_id {
            Some(id) => id.to_owned(),
            None => ConversationId::gen_new(),
        };

        let status = status.unwrap_or_default();
        let color = color.unwrap_or_else(|| crate::utils::id_to_color(&id));
        let pairwise = pairwise.unwrap_or(false);
        let muted = muted.unwrap_or(false);

        let expiration_period = expiration_period
            .unwrap_or_else(|| crate::config::db::preferred_expiration(tx).unwrap_or_default());

        let picture = match picture.take() {
            Some(picture) => {
                let path: std::path::PathBuf =
                    crate::image_utils::update_picture(picture, None::<&str>)?;
                path.into_os_string().into_string().ok()
            }
            None => None,
        };

        let last_active = Time::now();

        w!(tx.execute_named(
            include_str!("sql/add_conversation.sql"),
            named_params! {
                "@conversation_id": id,
                "@title": title,
                "@picture": picture,
                "@color": color,
                "@pairwise": pairwise,
                "@muted": muted,
                "@last_active_ts": last_active,
                "@expiration_period": expiration_period,
                "@status": status
            },
        ));

        Ok(Conversation {
            meta: ConversationMeta {
                conversation_id: id,
                title,
                color,
                pairwise,
                picture,
                last_active,
                expiration_period,
                muted,
                status,
            },
            members,
        })
    }

    pub(crate) fn add_nts(
        mut self,
        tx: &rusqlite::Transaction,
        local_id: UserId,
    ) -> Result<Conversation, HErr> {
        self.members.push(local_id);

        Ok(self.raw_add(tx)?)
    }

    pub(crate) fn add_tx(
        mut self,
        tx: &rusqlite::Transaction,
    ) -> Result<Conversation, HErr> {
        let local_id = crate::config::db::id(tx)?;

        if !self.member_set.contains(&local_id) {
            self.members.push(local_id);
        }

        let conv = self.raw_add(tx)?;
        crate::members::db::add_members_with_tx(tx, conv.meta.conversation_id, &conv.members)?;
        Ok(conv)
    }

    pub(crate) fn add_db(
        self,
        conn: &mut rusqlite::Connection,
    ) -> Result<Conversation, HErr> {
        let tx = w!(conn.transaction());
        let conv = w!(self.add_tx(&tx));
        w!(tx.commit());
        Ok(conv)
    }
}
