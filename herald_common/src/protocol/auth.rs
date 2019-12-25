use super::*;

pub type Method = u8;
pub const REGISTER: Method = 0;
pub const LOGIN: Method = 1;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AuthState {
    AwaitMethod,
    Login(LoginState),
    Register(RegisterState),
    Done(GlobalId),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RegisterState {
    CheckLoop,
    Done(GlobalId),
}

pub mod register {
    use super::*;

    #[derive(Ser, De, Debug, Clone, Copy, Eq, PartialEq)]
    pub enum ClientEvent {
        Check(UserId),
        Claim(Signed<UserId>),
    }

    #[derive(Ser, De, Debug, Clone, Copy, Eq, PartialEq)]
    pub enum ServeEvent {
        Taken,
        Available,
        Success,
        BadSig(SigValid),
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LoginState {
    AwaitClaim,
    Challenge(GlobalId),
    Accepted(GlobalId),
    Rejected,
}

pub mod login_types {
    use super::*;

    #[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ClaimResponse {
        Challenge,
        KeyInvalid,
        MissingUID,
    }

    // #[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
    // pub struct LoginToken(pub sig::Signature);

    // #[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
    // pub enum LoginTokenResponse {
    //     Success,
    //     BadSig,
    // }
}
