use super::*;
use rusqlite::named_params;
use std::ops::Not;

impl Search {
    pub(super) fn next_page_db(
        &mut self,
        conn: &mut rusqlite::Connection,
    ) -> Result<Option<Vec<SearchResult>>, HErr> {
        let mut stmt = conn.prepare_cached(include_str!("sql/get_page.sql"))?;

        let pattern = &self.pattern;
        let min_param = *self.min;

        let min = &mut self.min;

        let results = stmt.query_map_named(named_params!["@old_min": min_param], |row| {
            let time = row.get("insertion_ts")?;
            *min = time;

            let body: MessageBody = row.get("body")?;

            if pattern.is_match(body.as_str()).not() {
                return Ok(None);
            }

            Ok(Some(SearchResult {
                body,
                time,
                message_id: row.get("msg_id")?,
                author: row.get("author")?,
                conversation: row.get("conversation_id")?,
                has_attachments: row.get("has_attachments")?,
            }))
        })?;

        let mut out = Vec::new();

        // If it is true after processing the results,
        // we've processed all of the messages and should return `None`
        let mut done: bool = true;

        for res in results {
            done = false;
            if let Some(res) = res? {
                out.push(res);
            }
        }

        if done {
            return Ok(None);
        }

        Ok(Some(out))
    }
}
