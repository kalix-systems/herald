use super::*;
use search_pattern::SearchPattern;

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
    /// Indicates whether the message has attachments
    pub has_attachments: bool,
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

#[cfg(test)]
mod tests;
