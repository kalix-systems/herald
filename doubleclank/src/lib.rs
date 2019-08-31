use sodiumoxide::crypto::secretbox;

pub mod kx;
pub mod sym;
mod utils;

use utils::*;

pub struct ConversationId(u64);
pub struct MessageId(u64);

// pub struct RootChain {
//     key: RootKey,
// }

// impl KemChain {
//     fn receive_key(&mut self, rec: kx::PublicKey) -> ChainKey {
//         self.root_key = kx::server_session_keys(&self.pub_key, &self.sec_key, &rec)
//             .expect("key exchange failed");
//         self.recv_key = rec;
//     }
// }
