use super::*;
use std::collections::HashSet;

mod db;

/// Builder for Conversations
#[derive(Default)]
pub struct ConversationBuilder {
    /// Conversation id
    pub conversation_id: Option<ConversationId>,
    /// Conversation title
    pub title: Option<String>,
    /// Conversation picture
    pub picture: Option<String>,
    /// Conversation color,
    pub color: Option<u32>,
    /// Indicates whether the conversation is muted
    pub muted: Option<bool>,
    /// Indicates whether the conversation is a canonical pairwise conversation
    pub pairwise: Option<bool>,
    /// The default period that passes before a message expires
    pub expiration_period: Option<ExpirationPeriod>,
    /// Members to be added to the conversation
    members: Vec<UserId>,
    member_set: HashSet<UserId>,
}

impl ConversationBuilder {
    /// Creates new `ConversationBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets title
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title.replace(title);
        self
    }

    /// Gets members to be added to converation
    pub fn members(&self) -> &[UserId] {
        &self.members
    }

    /// Sets conversation id
    pub fn conversation_id(&mut self, cid: ConversationId) -> &mut Self {
        self.conversation_id.replace(cid);
        self
    }

    /// Sets picture
    pub fn picture(&mut self, picture: String) -> &mut Self {
        self.picture.replace(picture);
        self
    }

    /// Sets color
    pub fn color(&mut self, color: u32) -> &mut Self {
        self.color.replace(color);
        self
    }

    /// Sets muted status. This is meant to be used when syncing conversations between devices.
    pub fn muted(&mut self, muted: bool) -> &mut Self {
        self.muted.replace(muted);
        self
    }

    /// Sets whether or not conversation is pairwise.
    pub fn pairwise(&mut self, pairwise: bool) -> &mut Self {
        self.pairwise.replace(pairwise);
        self
    }

    /// Sets expiration period
    pub fn expiration_period(&mut self, expiration_period: ExpirationPeriod) -> &mut Self {
        self.expiration_period.replace(expiration_period);
        self
    }

    /// Adds a member to the conversation builder
    pub fn add_member(&mut self, uid: UserId) {
        if self.member_set.contains(&uid) {
            return;
        }

        self.members.push(uid);
        self.member_set.insert(uid);
    }

    /// Indicates whether a member has been added
    pub fn has_member(&self, uid: &UserId) -> bool {
        self.member_set.contains(uid)
    }

    /// Overrides member list with new members
    pub(crate) fn override_members(&mut self, members: Vec<UserId>) -> &mut Self {
        // De-duplicate
        self.member_set = members.into_iter().collect();
        self.members = self.member_set.iter().copied().collect();
        self
    }

    /// Removes a member from the conversation builder, if they are present.
    ///
    /// Returns the index of the member that has been removed
    pub fn remove_member(&mut self, uid: &UserId) -> Option<usize> {
        if self.member_set.remove(uid) {
            let pos = self.members.iter().position(|u| u == uid)?;
            self.members.remove(pos);
            Some(pos)
        } else {
            None
        }
    }

    /// Removes the member at index `ix`, if they are present.
    pub fn remove_member_by_index(&mut self, ix: usize) {
        if ix < self.members.len() {
            let uid = self.members.remove(ix);
            self.member_set.remove(&uid);
        }
    }

    /// Adds conversation to database
    pub fn add(self) -> Result<Conversation, HErr> {
        let mut db = Database::get()?;
        let conv = self.add_db(&mut db)?;
        Ok(conv)
    }
}
