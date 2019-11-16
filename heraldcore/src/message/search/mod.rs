use super::*;
use crate::utils::SearchPattern;

mod db;

type SearchIndex = Time;

/// Incrementally searches
pub struct Search {
    min: SearchIndex,
    pattern: SearchPattern,
}

impl Search {
    /// Creates a new `Search` handle with the provided `pattern`.
    pub fn new(pattern: SearchPattern) -> Self {
        Self {
            min: Time(std::i64::MAX),
            pattern,
        }
    }

    /// Fetches next page of search results.
    pub fn next_page(&mut self) -> Result<Option<Vec<SearchResult>>, HErr> {
        let mut conn = Database::get()?;
        self.next_page_db(&mut conn)
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
    pub body: MessageBody,
    /// Insertion time
    pub time: Time,
    /// Indicates whether the message has attachments
    pub has_attachments: bool,
}

#[cfg(test)]
mod tests;
