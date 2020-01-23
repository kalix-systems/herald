use super::*;
pub use coretypes::conversation::settings::*;
use network_types as nt;

pub(crate) mod db {
    use super::*;

    pub(crate) fn update_expiration(
        conn: &rusqlite::Connection,
        period: ExpirationPeriod,
        cid: &ConversationId,
    ) -> Result<nt::GroupSettingsUpdate, HErr> {
        use crate::conversation::db::*;
        use nt::GroupSettingsUpdate::*;

        set_expiration_period(&conn, cid, period)?;
        Ok(Expiration(period))
    }

    pub(crate) fn update_title(
        conn: &rusqlite::Connection,
        title: Option<String>,
        cid: &ConversationId,
    ) -> Result<nt::GroupSettingsUpdate, HErr> {
        use crate::conversation::db::*;
        use nt::GroupSettingsUpdate::*;

        set_title(&conn, cid, title.as_ref().map(String::as_str))?;
        Ok(Title(title))
    }

    pub(crate) fn update_picture(
        conn: &rusqlite::Connection,
        group_picture: Option<image_utils::ProfilePicture>,
        cid: &ConversationId,
    ) -> Result<(nt::GroupSettingsUpdate, Option<String>), HErr> {
        use crate::conversation::db::*;
        use nt::GroupSettingsUpdate::*;

        let path = set_picture(&conn, cid, group_picture)?;
        let buf = path.as_ref().map(std::fs::read).transpose()?;
        Ok((Picture(buf), path))
    }

    pub(crate) fn apply_inbound(
        conn: &rusqlite::Connection,
        update: nt::GroupSettingsUpdate,
        cid: &ConversationId,
    ) -> Result<SettingsUpdate, HErr> {
        use crate::conversation::db::*;
        use nt::GroupSettingsUpdate::*;

        match update {
            Expiration(period) => {
                set_expiration_period(&conn, cid, period)?;
                Ok(SettingsUpdate::Expiration(period))
            }
            Title(title) => {
                set_title(&conn, cid, title.as_ref().map(String::as_str))?;
                Ok(SettingsUpdate::Title(title))
            }
            Picture(bytes) => {
                let path = set_picture_buf(&conn, cid, bytes.as_ref().map(Vec::as_slice))?;
                Ok(SettingsUpdate::Picture(path))
            }
        }
    }
}
