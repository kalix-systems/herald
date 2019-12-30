use super::*;
use coremacros::w;
use rusqlite::named_params;

impl Search {
    pub(super) fn next_page_db(
        &mut self,
        conn: &mut rusqlite::Connection,
    ) -> Result<Option<Vec<SearchResult>>, HErr> {
        let mut stmt = w!(conn.prepare_cached(include_str!("sql/get_page.sql")));

        let pattern = &self.pattern;
        let min_param = self.min;

        let min = &mut self.min;

        let results = w!(stmt.query_map_named(
            named_params!("@old_min_time": min_param.time, "@old_row_id": min_param.row_id),
            |row| {
                let time = row.get("insertion_ts")?;
                let row_id = row.get("rowid")?;
                let message_id = row.get("msg_id")?;
                let author = row.get("author")?;
                let conversation = row.get("conversation_id")?;
                let body: MessageBody = row.get("body")?;

                *min = Index { time, row_id };

                let body = match ResultBody::from_match(pattern, &body) {
                    Some(body) => body,
                    None => return Ok(None),
                };

                Ok(Some(SearchResult {
                    body,
                    time,
                    message_id,
                    author,
                    conversation,
                    rowid: row_id,
                }))
            },
        ));

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
