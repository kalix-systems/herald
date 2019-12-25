use super::*;
use herald_common::protocol::auth::*;
use krpc::*;

impl State {
    pub async fn auth_transition<Tx, Rx>(
        &self,
        state: AuthState,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<AuthState, anyhow::Error>
    where
        Tx: AsyncWrite + Unpin,
        Rx: AsyncRead + Unpin,
    {
        use AuthState::*;
        match state {
            Done(g) => Ok(Done(g)),
            Register(register) => match self.register_transition(register, tx, rx).await? {
                RegisterState::Done(g) => Ok(Done(g)),
                r => Ok(Register(r)),
            },
            Login(login) => match self.login_transition(login, tx, rx).await? {
                LoginState::Done(g) => Ok(Done(g)),
                l => Ok(Login(l)),
            },
            AwaitMethod => {
                let method = rx.read_u8().await?;
                match method {
                    REGISTER => Ok(Register(RegisterState::CheckLoop)),
                    LOGIN => Ok(Login(LoginState::AwaitClaim)),
                    m => Err(anyhow!(format!("unknown method {}", m))),
                }
            }
        }
    }
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
        use login_types::*;
        use LoginState::*;

        match login {
            Done(g) => Ok(Done(g)),
            Rejected => Err(anyhow!("login rejected")),
            AwaitClaim => {
                let mut conn = self.new_connection().await?;
                let did: sig::PublicKey = rx.read_de().await?;
                if !conn.key_is_valid(did).await? {
                    tx.write_ser(&ClaimResponse::KeyInvalid).await?;
                    Ok(AwaitClaim)
                } else if let Some(uid) = conn.user_of(did).await? {
                    tx.write_ser(&ClaimResponse::Challenge).await?;
                    Ok(Challenge(GlobalId { uid, did }))
                } else {
                    tx.write_ser(&ClaimResponse::MissingUID).await?;
                    Ok(AwaitClaim)
                }
            }
            Challenge(g) => {
                let challenge = UQ::gen_new();
                tx.write_all(challenge.as_ref()).await?;
                let sig = rx.read_de().await?;
                if g.did.verify(challenge.as_ref(), sig) {
                    Ok(Done(g))
                } else {
                    Ok(Rejected)
                }
            }
        }
    }

    pub async fn register_transition<Tx, Rx>(
        &self,
        register: RegisterState,
        tx: &mut Framed<Tx>,
        rx: &mut Framed<Rx>,
    ) -> Result<RegisterState, anyhow::Error>
    where
        Tx: AsyncWrite + Unpin,
        Rx: AsyncRead + Unpin,
    {
        use register::*;
        use RegisterState::*;

        match register {
            CheckLoop => match rx.read_de().await? {
                ClientEvent::Check(u) => {
                    if self
                        .new_connection()
                        .await?
                        .recip_exists(Recip::One(SingleRecip::User(u)))
                        .await?
                    {
                        tx.write_ser(&ServeEvent::Taken).await?;
                        Ok(CheckLoop)
                    } else {
                        tx.write_ser(&ServeEvent::Available).await?;
                        Ok(CheckLoop)
                    }
                }
                ClientEvent::Claim(s) => {
                    let valid = s.verify_sig();
                    if valid == SigValid::Yes {
                        let res = self.new_connection().await?.new_user(s).await?;
                        tx.write_ser(&res).await?;
                        Ok(match res {
                            ServeEvent::Success => Done(GlobalId {
                                uid: *s.data(),
                                did: *s.signed_by(),
                            }),
                            _ => CheckLoop,
                        })
                    } else {
                        tx.write_ser(&ServeEvent::BadSig(valid)).await?;
                        Ok(CheckLoop)
                    }
                }
            },
            Done(g) => Ok(Done(g)),
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
