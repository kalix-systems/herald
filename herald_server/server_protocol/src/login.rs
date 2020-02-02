use super::*;

impl State {
    pub async fn auth_transition<Tx, Rx, E>(
        &self,
        state: AuthState,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> Result<AuthState, anyhow::Error>
    where
        Tx: Sink<Bytes> + Unpin,
        <Tx as Sink<Bytes>>::Error: std::error::Error + Send + Sync + 'static,
        Rx: Stream<Item = Result<Vec<u8>, E>> + Unpin,
        E: std::error::Error + Send + Sync + 'static,
    {
        use AuthState::*;
        match state {
            Done(g) => Ok(Done(g)),
            Register(register) => Ok(match self.register_transition(register, tx, rx).await? {
                RegisterState::Done(g) => Done(g),
                r => Register(r),
            }),
            Login(login) => Ok(match self.login_transition(login, tx, rx).await? {
                LoginState::Done(g) => Done(g),
                l => Login(l),
            }),
            AwaitMethod => {
                let raw_method = rx
                    .next()
                    .await
                    .transpose()?
                    .ok_or_else(|| anyhow!("failed to await auth method"))?;
                match kson::from_bytes(raw_method.into())? {
                    REGISTER => Ok(Register(RegisterState::CheckLoop)),
                    LOGIN => Ok(Login(LoginState::AwaitClaim)),
                    m => Err(anyhow!(format!("unknown method {}", m))),
                }
            }
        }
    }

    pub async fn login_transition<Tx, Rx, E>(
        &self,
        login: LoginState,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> Result<LoginState, anyhow::Error>
    where
        Tx: Sink<Bytes> + Unpin,
        <Tx as Sink<Bytes>>::Error: std::error::Error + Send + Sync + 'static,
        Rx: Stream<Item = Result<Vec<u8>, E>> + Unpin,
        E: std::error::Error + Send + Sync + 'static,
    {
        use login_types::*;
        use LoginState::*;

        match login {
            Done(g) => Ok(Done(g)),
            Rejected => Err(anyhow!("login rejected")),
            AwaitClaim => {
                let did: sig::PublicKey = read_de(rx).await?;
                let mut conn = self.new_connection().await?;
                if !conn.key_is_valid(did).await? {
                    drop(conn);
                    write_ser(tx, &ClaimResponse::KeyInvalid).await?;
                    Ok(AwaitClaim)
                } else if let Some(uid) = conn.user_of(did).await? {
                    drop(conn);
                    write_ser(tx, &ClaimResponse::Challenge).await?;
                    Ok(Challenge(GlobalId { uid, did }))
                } else {
                    drop(conn);
                    write_ser(tx, &ClaimResponse::MissingUID).await?;
                    Ok(AwaitClaim)
                }
            }
            Challenge(g) => {
                let challenge = UQ::gen_new();
                write_ser(tx, &challenge).await?;
                let sig = read_de(rx).await?;
                if g.did.verify(challenge.as_ref(), sig) {
                    write_ser(tx, &ChallengeResult::Success).await?;
                    Ok(Done(g))
                } else {
                    write_ser(tx, &ChallengeResult::Failed).await?;
                    Ok(Rejected)
                }
            }
        }
    }

    pub async fn register_transition<Tx, Rx, E>(
        &self,
        register: RegisterState,
        tx: &mut Tx,
        rx: &mut Rx,
    ) -> Result<RegisterState, anyhow::Error>
    where
        Tx: Sink<Bytes> + Unpin,
        <Tx as Sink<Bytes>>::Error: std::error::Error + Send + Sync + 'static,
        Rx: Stream<Item = Result<Vec<u8>, E>> + Unpin,
        E: std::error::Error + Send + Sync + 'static,
    {
        use register::*;
        use RegisterState::*;

        match register {
            CheckLoop => match read_de(rx).await? {
                ClientEvent::Check(u) => {
                    if self
                        .new_connection()
                        .await?
                        .recip_exists(Recip::One(SingleRecip::User(u)))
                        .await?
                    {
                        write_ser(tx, &ServeEvent::Taken).await?;
                        Ok(CheckLoop)
                    } else {
                        write_ser(tx, &ServeEvent::Available).await?;
                        Ok(CheckLoop)
                    }
                }
                ClientEvent::Claim(s) => {
                    let valid = s.verify_sig();
                    if valid == SigValid::Yes {
                        let res = self.new_connection().await?.new_user(s).await?;
                        write_ser(tx, &res).await?;
                        Ok(match res {
                            ServeEvent::Success => Done(GlobalId {
                                uid: *s.data(),
                                did: *s.signed_by(),
                            }),
                            _ => CheckLoop,
                        })
                    } else {
                        write_ser(tx, &ServeEvent::BadSig(valid)).await?;
                        Ok(CheckLoop)
                    }
                }
            },
            Done(g) => Ok(Done(g)),
        }
    }
}
