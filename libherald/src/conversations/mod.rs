use crate::{cont_none, err, interface::*, none};
use heraldcore::{
    conversation::{self, ConversationMeta, ExpirationPeriod},
    types::ConversationId,
};
use im::vector::Vector;
use search_pattern::SearchPattern;

pub(crate) mod shared;
use shared::*;
mod handlers;
mod imp;
mod trait_imp;
pub(crate) mod types;
pub(crate) mod underscore;
use types::*;

/// A wrapper around a vector of `Conversation`, with additional
/// fields to facilitate interaction with Qt.
pub struct Conversations {
    emit: ConversationsEmitter,
    model: ConversationsList,
    filter: Option<SearchPattern>,
    filter_regex: bool,
    list: Vector<Conversation>,
    loaded: bool,
}
