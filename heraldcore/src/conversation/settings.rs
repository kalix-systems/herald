use super::*;
pub use coretypes::conversation::settings::*;

/// Applies settings update to the conversation with the id `cid`.
pub fn apply(
    update: &SettingsUpdate,
    cid: &ConversationId,
) -> Result<(), HErr> {
    let conn = Database::get()?;
    db::apply(&conn, update, cid)
}

/// Sends the update to the conversation
pub fn send_update(
    update: SettingsUpdate,
    cid: &ConversationId,
) -> Result<(), HErr> {
    use crate::network::send_conversation_settings_update;

    send_conversation_settings_update(*cid, update)
}

pub(crate) mod db {
    use super::*;

    pub(crate) fn apply(
        conn: &rusqlite::Connection,
        update: &SettingsUpdate,
        cid: &ConversationId,
    ) -> Result<(), HErr> {
        use crate::conversation::db::*;
        use SettingsUpdate::*;

        match update {
            Expiration(period) => Ok(set_expiration_period(&conn, cid, *period)?),
            Title(title) => Ok(set_title(&conn, cid, title.as_ref().map(String::as_str))?),
            Color(color) => Ok(set_color(&conn, cid, *color)?),
            _ => Ok(()),
        }
    }

    pub(crate) fn update_expiration(
        conn: &rusqlite::Connection,
        period: ExpirationPeriod,
        cid: &ConversationId,
    ) -> Result<cmessages::GroupSettingsUpdate, HErr> {
        use crate::conversation::db::*;
        use cmessages::GroupSettingsUpdate::*;

        set_expiration_period(&conn, cid, period)?;
        Ok(Expiration(period))
    }

    pub(crate) fn update_title(
        conn: &rusqlite::Connection,
        title: Option<String>,
        cid: &ConversationId,
    ) -> Result<cmessages::GroupSettingsUpdate, HErr> {
        use crate::conversation::db::*;
        use cmessages::GroupSettingsUpdate::*;

        set_title(&conn, cid, title.as_ref().map(String::as_str))?;
        Ok(Title(title))
    }

    pub(crate) fn update_color(
        conn: &rusqlite::Connection,
        color: u32,
        cid: &ConversationId,
    ) -> Result<cmessages::GroupSettingsUpdate, HErr> {
        use crate::conversation::db::*;
        use cmessages::GroupSettingsUpdate::*;

        set_color(&conn, cid, color)?;
        Ok(Color(color))
    }

    pub(crate) fn update_picture(
        conn: &rusqlite::Connection,
        group_picture: Option<image_utils::ProfilePicture>,
        cid: &ConversationId,
    ) -> Result<(cmessages::GroupSettingsUpdate, Option<String>), HErr> {
        use crate::conversation::db::*;
        use cmessages::GroupSettingsUpdate::*;

        let path = set_picture(&conn, cid, group_picture)?;
        let buf = path.as_ref().map(std::fs::read).transpose()?;
        Ok((Picture(buf), path))
    }

    pub(crate) fn apply_inbound(
        conn: &rusqlite::Connection,
        update: cmessages::GroupSettingsUpdate,
        cid: &ConversationId,
    ) -> Result<SettingsUpdate, HErr> {
        use crate::conversation::db::*;
        use cmessages::GroupSettingsUpdate::*;

        match update {
            Expiration(period) => {
                set_expiration_period(&conn, cid, period)?;
                Ok(SettingsUpdate::Expiration(period))
            }
            Title(title) => {
                set_title(&conn, cid, title.as_ref().map(String::as_str))?;
                Ok(SettingsUpdate::Title(title))
            }
            Color(color) => {
                set_color(&conn, cid, color)?;
                Ok(SettingsUpdate::Color(color))
            }
            Picture(bytes) => {
                let path = set_picture_buf(&conn, cid, bytes.as_ref().map(Vec::as_slice))?;
                Ok(SettingsUpdate::Picture(path))
            }
        }
    }
}
