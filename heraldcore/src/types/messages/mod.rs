use super::*;
use crate::{
    chainkeys::DecryptionResult, config::*, errors::HErr::*, message::attachments::Attachment,
};
use chainmail::{block::*, errors::ChainError::CryptoError};
use std::convert::AsRef;

/// Types relevant to [`ConversationMessage`]s
pub(crate) mod cmessages;
/// Types associated with [`DeviceMessage`]s
pub(crate) mod dmessages;

mod rusqlite_imp;

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
    ContactReqAck(cmessages::ContactReqAck),
    /// A normal message.
    Msg(cmessages::Msg),
    /// An acknowledgement of a normal message.
    Ack(cmessages::Ack),
    /// An update to the conversation settings
    Settings(crate::conversation::settings::SettingsUpdate),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// A conversation message
pub struct ConversationMessage {
    /// The ciphertext of the message. After decryption, this should deserialize as a
    /// [`ConversationMessageBody`].
    body: chainmail::block::Block,
    /// Conversation the message is associated with
    cid: ConversationId,
    /// Who supposedly sent the message
    from: GlobalId,
}

impl ConversationMessage {
    /// Raw body of the message
    pub fn body(&self) -> &Block {
        &self.body
    }

    /// `ConversationId` associated with the message
    pub fn cid(&self) -> ConversationId {
        self.cid
    }

    /// The device the message claims to be from.
    pub fn from(&self) -> GlobalId {
        self.from
    }

    /// Seals the messages.
    pub fn seal(
        // note: this is only mut because BlockStore thinks it should be
        cid: ConversationId,
        content: &ConversationMessageBody,
    ) -> Result<(ConversationMessage, BlockHash, ChainKey), HErr> {
        let cbytes = serde_cbor::to_vec(content)?;
        let kp = Config::static_keypair()?;
        let from = Config::static_gid()?;
        let (hashes, keys) = cid.get_unused()?.into_iter().unzip();
        let channel_key = cid.get_channel_key()?;

        let SealData { block, key } =
            Block::seal(kp.secret_key(), &channel_key, &keys, hashes, cbytes).ok_or(CryptoError)?;
        let hash = block.compute_hash().ok_or(CryptoError)?;

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
    pub fn open(self) -> Result<Vec<(ConversationMessageBody, GlobalId)>, HErr> {
        let ConversationMessage { cid, from, body } = self;

        let mut out = Vec::new();

        let mut blocks = {
            match cid.open_block(&from, body)? {
                DecryptionResult::Success(bvec, unlocked) => {
                    out.push((serde_cbor::from_slice(&bvec)?, from));
                    unlocked
                }
                DecryptionResult::Pending => Vec::new(),
            }
        };

        while let Some((block, from)) = blocks.pop() {
            match cid.open_block(&from, block)? {
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
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
/// Types of device message.
pub enum DeviceMessageBody {
    /// A contact request
    ContactReq(dmessages::ContactReq),
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
/// A message sent to a specific device.
pub struct DeviceMessage {
    /// The sender of the message
    from: GlobalId,
    /// The ciphertext of the message. After decryption, this should deserialize as a
    /// [`DeviceMessageBody`]
    content: Vec<u8>,
    nonce: box_::Nonce,
    tag: box_::Tag,
    /// The prekey used to encrypt the message.
    /// If none, the message was encrypted with this device's public signing key treated as an
    /// encryption key.
    /// Until we've implemented prekey infrastructure, this will always be `None`.
    prekey: Option<box_::PublicKey>,
}

fn spk_to_epk(pk: &sig::PublicKey) -> Result<box_::PublicKey, HErr> {
    let mut pkbuf = [0u8; box_::PUBLICKEYBYTES];
    let ret = unsafe {
        libsodium_sys::crypto_sign_ed25519_pk_to_curve25519(
            pkbuf.as_mut_ptr(),
            pk.as_ref().as_ptr(),
        )
    };
    if ret == 0 {
        Ok(box_::PublicKey(pkbuf))
    } else {
        Err(HeraldError(
            "failed to convert ed25519 public key to x25519 public key".into(),
        ))
    }
}
fn ssk_to_esk(sk: &sign::SecretKey) -> Result<box_::SecretKey, HErr> {
    let mut skbuf = [0u8; box_::SECRETKEYBYTES];
    let ret = unsafe {
        libsodium_sys::crypto_sign_ed25519_sk_to_curve25519(
            skbuf.as_mut_ptr(),
            sk.as_ref().as_ptr(),
        )
    };
    if ret == 0 {
        Ok(box_::SecretKey(skbuf))
    } else {
        Err(HeraldError(
            "failed to convert ed25519 public key to x25519 public key".into(),
        ))
    }
}

impl DeviceMessage {
    pub(crate) fn seal(
        to: &sig::PublicKey,
        content: &DeviceMessageBody,
    ) -> Result<DeviceMessage, HErr> {
        let mut content = serde_cbor::to_vec(content)?;

        let pk = spk_to_epk(to)?;

        let kp = Config::static_keypair()?;
        let sk = ssk_to_esk(kp.secret_key())?;

        let nonce = box_::gen_nonce();

        let tag = box_::seal_detached(&mut content, &nonce, &pk, &sk);

        Ok(DeviceMessage {
            from: Config::static_gid()?,
            content,
            nonce,
            tag,
            prekey: None,
        })
    }

    pub(crate) fn open(self) -> Result<(GlobalId, DeviceMessageBody), HErr> {
        // TODO: remove this, handle prekey
        assert!(self.prekey.is_none());

        let DeviceMessage {
            from,
            mut content,
            nonce,
            tag,
            ..
        } = self;

        let pk = spk_to_epk(&from.did)?;

        let kp = Config::static_keypair()?;
        let sk = ssk_to_esk(kp.secret_key())?;

        box_::open_detached(&mut content, &tag, &nonce, &pk, &sk)
            .map_err(|_| HeraldError("Failed to decrypt message to device".into()))?;

        let dm = serde_cbor::from_slice(&content)?;

        Ok((from, dm))
    }
}
