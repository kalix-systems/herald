use super::*;
use crate::w;
use rusqlite::named_params;
use search_pattern::Captures;
use std::ops::Not;
use unicode_segmentation::UnicodeSegmentation;

const START_TAG: &str = "<b>";
const END_TAG: &str = "</b>";

fn process(
    pattern: &SearchPattern,
    body: &str,
    take_num: usize,
    rev: bool,
) -> String {
    let body: String = if rev {
        let rev_body: String = UnicodeSegmentation::graphemes(body, true)
            .rev()
            .take(take_num)
            .collect();

        rev_body.chars().rev().collect()
    } else {
        UnicodeSegmentation::graphemes(body, true)
            .take(take_num)
            .collect()
    };

    pattern
        .replace_all(&body, |caps: &Captures| {
            format!(
                "{}{}{}",
                START_TAG,
                caps.get(0).map(|m| m.as_str()).unwrap_or(""),
                END_TAG
            )
        })
        .to_string()
}

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
                let has_attachments = row.get("has_attachments")?;
                let body: MessageBody = row.get("body")?;

                *min = Index { time, row_id };

                if pattern.is_match(body.as_str()).not() {
                    return Ok(None);
                }

                let body = {
                    let p_match = match pattern.find(body.as_str()) {
                        Some(p_match) => p_match,
                        None => {
                            return Ok(None);
                        }
                    };

                    let (before_first, tail) = body.as_str().split_at(p_match.start());
                    let (first_match, after_first) = tail.split_at(p_match.end() - p_match.start());

                    let before_first = process(pattern, before_first, 70, true);
                    let first_match = process(pattern, first_match, 40, false);
                    let after_first = process(pattern, after_first, 70, false);

                    ResultBody {
                        before_first,
                        first_match,
                        after_first,
                    }
                };

                Ok(Some(SearchResult {
                    body,
                    time,
                    message_id,
                    author,
                    conversation,
                    has_attachments,
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
