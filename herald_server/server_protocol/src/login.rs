use super::*;
use herald_common::protocol::auth::*;
use krpc::*;

impl State {
    pub async fn login_transition<Tx, Rx>(
        &self,
        login: LoginState,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<LoginState, anyhow::Error>
    where
        Tx: AsyncWrite + Unpin,
        Rx: AsyncRead + Unpin,
    {
        // use login::{ClientEvent::*, ServeEvent::*};
        use LoginState::*;
        match login {
            Accepted(g) => Ok(Accepted(g)),
            Rejected => Err(anyhow!("login rejected")),
            AwaitClaim => Ok(Challenge(rx.read_de().await?)),
            Challenge(uid) => {
                let challenge = UQ::gen_new();
                tx.write_all(challenge.as_ref()).await?;
                let sig: SigMeta = rx.read_de().await?;
                let valid = sig.verify_sig(challenge.as_ref());
                tx.write_ser(&valid).await?;
                if valid == SigValid::Yes
                    && self
                        .new_connection()
                        .await?
                        .user_of(*sig.signed_by())
                        .await?
                        == Some(uid)
                {
                    Ok(Accepted(GlobalId {
                        uid,
                        did: *sig.signed_by(),
                    }))
                } else {
                    Ok(Rejected)
                }
            }
        }
    }
}
// impl State {
//     pub async fn auth_transition(
//         &self,
//         auth: AuthState,
//         tx: &mut Framed<Tx>,
//         rx: &mut Framed<Rx>,
//     ) -> Result<AuthState, Error> {
//     }
// }
// impl ServerState {
//     pub async fn transition<Tx, Rx>(
//         self,
//         tx: &mut Framed<Tx>,
//         rx: &mut Framed<Rx>,
//     ) -> Result<Self, Error>
//     where
//         Tx: AsyncWrite + Unpin,
//         Rx: AsyncRead + Unpin,
//     {
//         match self {
//             AwaitMethod => match rx.read_u8().await? {},
//         }
//     }
// }
