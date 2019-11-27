use super::*;
use rusqlite::named_params;

impl ConversationBuilder {
    fn raw_add(
        self,
        tx: &rusqlite::Transaction,
    ) -> Result<Conversation, HErr> {
        let Self {
            conversation_id,
            title,
            picture,
            color,
            muted,
            pairwise,
            expiration_period,
            members,
            ..
        } = self;

        let id = match conversation_id {
            Some(id) => id.to_owned(),
            None => ConversationId::gen_new(),
        };

        let color = color.unwrap_or_else(|| crate::utils::id_to_color(&id));
        let pairwise = pairwise.unwrap_or(false);
        let muted = muted.unwrap_or(false);
        let expiration_period = expiration_period.unwrap_or_default();

        let picture = match picture.as_ref() {
            Some(picture) => {
                // TODO Give more specific error
                let path: std::path::PathBuf = crate::image_utils::update_picture(picture, None)?;
                path.into_os_string().into_string().ok()
            }
            None => None,
        };

        let last_active = Time::now();

        tx.execute_named(
            include_str!("sql/add_conversation.sql"),
            named_params! {
                "@conversation_id": id,
                "@title": title,
                "@picture": picture,
                "@color": color,
                "@pairwise": pairwise,
                "@muted": muted,
                "@last_active_ts": last_active,
                "@expiration_period": expiration_period
            },
        )?;

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
        let tx = conn.transaction()?;
        let conv = self.add_tx(&tx)?;
        tx.commit()?;
        Ok(conv)
    }
}
