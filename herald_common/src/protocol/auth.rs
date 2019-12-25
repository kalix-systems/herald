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
    Claim(Signed<UserId>),
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
        Claimed,
        Failed(PKIResponse),
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LoginState {
    AwaitClaim,
    Challenge(UserId),
    Accepted(GlobalId),
    Rejected,
}

// pub mod login {
//     use super::*;

//     // #[derive(Ser, De, Debug, Clone, Copy, Eq, PartialEq)]
//     // pub enum ClientEvent {
//     //     Claim(UserId),
//     //     Answer(SigMeta),
//     // }

//     // #[derive(Ser, De, Debug, Clone, Copy, Eq, PartialEq)]
//     // pub enum ServeEvent {
//     //     UnknownUID,
//     //     UnknownKey,
//     //     Challenge(UQ),
//     //     Failed(SigValid),
//     // }
// }
