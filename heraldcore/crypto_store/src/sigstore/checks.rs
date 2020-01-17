use super::*;

impl<'conn> Conn<'conn> {
    pub(super) fn key_deprecated(
        &self,
        key: &PK,
        of: &UserId,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = st!(self, "sigchain", "key_deprecated");

        let params = np!("@key": key.as_ref(), "@user_id": of);
        stmt.query_row_named(params, |row| row.get(0))
    }

    pub(super) fn key_endorsed(
        &self,
        key: &PK,
        of: &UserId,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = st!(self, "sigchain", "key_endorsed");

        let params = np!("@key": key.as_ref(), "@user_id": of);
        stmt.query_row_named(params, |row| row.get(0))
    }

    pub(super) fn key_is_genesis(
        &self,
        key: &PK,
        of: &UserId,
    ) -> Result<bool, rusqlite::Error> {
        let mut stmt = st!(self, "sigchain", "key_is_genesis");

        let params = np!("@key": key.as_ref(), "@user_id": of);
        stmt.query_row_named(params, |row| row.get(0))
    }
}
