use super::*;
use search_pattern::Captures;
use search_pattern::SearchPattern;
use unicode_segmentation::UnicodeSegmentation;

mod db;

#[derive(Clone, Copy)]
struct Index {
    time: Time,
    row_id: i64,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            time: Time::from(std::i64::MAX),
            row_id: std::i64::MAX,
        }
    }
}

/// Incrementally searches
pub struct Search {
    min: Index,
    /// The search pattern being used
    pub pattern: SearchPattern,
}

impl Search {
    /// Creates a new `Search` handle with the provided `pattern`.
    pub fn new(pattern: SearchPattern) -> Self {
        Self {
            min: Index::default(),
            pattern,
        }
    }

    /// Fetches next page of search results.
    pub fn next_page(&mut self) -> Result<Option<Vec<SearchResult>>, HErr> {
        let mut conn = Database::get()?;
        self.next_page_db(&mut conn)
    }

    /// Replaces the search pattern, and resets the search.
    pub fn replace_pattern(
        &mut self,
        pattern: SearchPattern,
    ) {
        self.min = Index::default();
        self.pattern = pattern;
    }

    /// Resets the search without clearing the pattern
    pub fn reset_search(&mut self) {
        self.min = Index::default();
    }
}

#[derive(Clone, Debug)]
/// A search result produced by a global message search
pub struct SearchResult {
    /// Message id
    pub message_id: MsgId,
    /// Author user id
    pub author: UserId,
    /// Recipient user id
    pub conversation: ConversationId,
    /// Body of message
    pub body: ResultBody,
    /// Insertion time
    pub time: Time,
    rowid: i64,
}

/// Text of search result
#[derive(Clone, Debug)]
pub struct ResultBody {
    /// String before first match
    pub before_first: String,
    /// First match
    pub first_match: String,
    /// String after first match
    pub after_first: String,
}

impl ResultBody {
    fn from_match(
        pattern: &SearchPattern,
        body: &MessageBody,
    ) -> Option<ResultBody> {
        let add_tags = |text| {
            pattern
                .replace_all(text, |caps: &Captures| {
                    format!(
                        "{}{}{}",
                        START_TAG,
                        caps.get(0).map(|m| m.as_str()).unwrap_or(""),
                        END_TAG
                    )
                })
                .to_string()
        };

        let p_match = pattern.find(body.as_str())?;

        let (before, tail) = body.as_str().split_at(p_match.start());
        let (first, after) = tail.split_at(p_match.end() - p_match.start());

        let mut graphemes_used = 0;

        let first_match: String = UnicodeSegmentation::graphemes(first, true)
            .take(MATCH_CHARS)
            .map(|g| {
                graphemes_used += 1;
                g
            })
            .collect();

        let before_first: String = {
            let rev: String = UnicodeSegmentation::graphemes(before, true)
                .rev()
                .take((TOTAL_CHARS - graphemes_used).min(TOTAL_CHARS - MATCH_CHARS))
                .map(|g| {
                    graphemes_used += 1;
                    g
                })
                .collect();
            rev.chars().rev().collect()
        };

        let after_first: String = UnicodeSegmentation::graphemes(after, true)
            .take(TOTAL_CHARS - graphemes_used)
            .collect();

        let before_first = add_tags(&before_first);
        let first_match = add_tags(&first_match);
        let after_first = add_tags(&after_first);

        Some(ResultBody {
            before_first,
            first_match,
            after_first,
        })
    }
}

const START_TAG: &str = "<b>";
const END_TAG: &str = "</b>";

const TOTAL_CHARS: usize = 60;
const MATCH_CHARS: usize = 40;

#[cfg(test)]
mod tests;
