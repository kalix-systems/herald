use super::*;
use crate::message::{MessageBody, MessageReceiptStatus};
use network_types::cmessages::ConversationMessage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// The body of a [`ConversationMessage`]
pub enum ConversationMessageBody {
    /// A new key
    NewKey(cmessages::NewKey),
    /// A key to be marked as deprecated
    DepKey(cmessages::DepKey),
    /// Members just added to a conversation
    NewMembers(cmessages::NewMembers),
    /// A message a user receives upon being added to a conversation
    AddedToConvo(Box<cmessages::AddedToConvo>),
    /// An acknowledgement of a contact request.
    UserReqAck(cmessages::UserReqAck),
    /// A normal message.
    Msg(cmessages::Msg),
    /// An acknowledgement of a normal message.
    Ack(cmessages::Ack),
    /// An update to the conversation settings
    Settings(crate::conversation::settings::SettingsUpdate),
}

/// Seals the messages.
pub fn seal(
    // note: this is only mut because BlockStore thinks it should be
    cid: ConversationId,
    content: &ConversationMessageBody,
) -> Result<(ConversationMessage, BlockHash, ChainKey), HErr> {
    use chainkeys::ChainKeysError;

    let cbytes = serde_cbor::to_vec(content)?;
    let kp = config::keypair()?;
    let from = config::gid()?;
    let (hashes, keys) = chainkeys::get_unused(&cid)?.into_iter().unzip();
    let channel_key = chainkeys::get_channel_key(&cid)?;

    // FIXME: This is terrible. Why don't these functions just return results?
    let SealData { block, key } = Block::seal(kp.secret_key(), &channel_key, &keys, hashes, cbytes)
        .ok_or(HErr::ChainError(ChainKeysError::Chain(
            ChainMailError::CryptoError,
        )))?;
    let hash = block
        .compute_hash()
        .ok_or(HErr::ChainError(ChainKeysError::Chain(
            ChainMailError::CryptoError,
        )))?;

    Ok((
        ConversationMessage {
            cid,
            from,
            body: block,
        },
        hash,
        key,
    ))
}

/// Opens the message.
pub fn open(
    message: ConversationMessage,
) -> Result<Vec<(ConversationMessageBody, GlobalId)>, HErr> {
    let ConversationMessage { cid, from, body } = message;

    let mut out = Vec::new();

    let mut blocks = {
        match chainkeys::open_block(&cid, &from, body)? {
            DecryptionResult::Success(bvec, unlocked) => {
                out.push((serde_cbor::from_slice(&bvec)?, from));
                unlocked
            }
            DecryptionResult::Pending => Vec::new(),
        }
    };

    while let Some((block, from)) = blocks.pop() {
        match chainkeys::open_block(&cid, &from, block)? {
            DecryptionResult::Success(bvec, mut unlocked) => {
                blocks.append(&mut unlocked);
                out.push((serde_cbor::from_slice(&bvec)?, from));
            }
            DecryptionResult::Pending => {
                panic!("this should never happen");
            }
        }
    }

    Ok(out)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A new, signed key.
pub struct NewKey(pub Signed<sig::PublicKey>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A key that is to be marked as deprecated.
pub struct DepKey(pub Signed<sig::PublicKey>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Members that have just been added to a conversation.
pub struct NewMembers(pub Vec<UserId>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A message received by a user when they are addeded to a conversation.
pub struct AddedToConvo {
    /// The current members in that conversation.
    pub members: Vec<UserId>,
    /// The [`ConversationId`]
    pub cid: ConversationId,
    /// The conversation's title.
    pub title: Option<String>,
    /// The conversation's picture (as bytes)
    pub picture: Option<Vec<u8>>,
    /// The conversation's initial expiration period
    pub expiration_period: conversation::ExpirationPeriod,
    /// The genesis block for the new conversation
    pub gen: Genesis,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// An acknowledgement of a user request, with a bool to indicate whether the
/// request was accepted.
pub struct UserReqAck(pub bool);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A normal message to the conversation.
pub struct Msg {
    /// The message id. Globally unique.
    pub mid: MsgId,
    /// The content of the message.
    pub content: Message,
    /// The message id of the message being replied to, if this
    /// message is a reply.
    pub op: Option<MsgId>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Variants of messages.
pub struct Message {
    /// Body of the message
    pub body: Option<MessageBody>,
    /// Attachments
    pub attachments: Vec<Attachment>,
    /// Expiration time of the message
    pub expiration: Option<Time>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// An acknowledgement that a message was received.
pub struct Ack {
    /// The message id.
    pub of: MsgId,
    /// The receipt status of the message.
    pub stat: MessageReceiptStatus,
}
