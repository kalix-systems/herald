use super::*;

pub mod keys_of {
    use super::*;

    /// [`UserId`] to fetch keys of
    pub type Req = UserId;

    /// [`UserMeta`] found for requested [`UserId`], `None` where the user was not found.
    pub type Res = Option<UserMeta>;
}

pub mod key_info {
    use super::*;

    /// [`sig::PublicKey`] to get info of.
    pub type Req = sig::PublicKey;

    /// [`sig::PKMeta`] found for requested [`sig::PublicKey`], `None` where the key was not
    /// found.
    pub type Res = Option<sig::PKMeta>;
}

pub mod user_exists {
    use super::*;

    /// [`UserId`] to check existence of
    pub type Req = UserId;
    /// `true` if requested user exists, false otherwise
    pub type Res = bool;
}

pub mod push_users {
    use super::*;

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub struct Req {
        pub to: Vec<UserId>,
        pub exc: sig::PublicKey,
        pub msg: Bytes,
    }

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Success,
        Missing(Vec<UserId>),
    }
}

pub mod push_devices {
    use super::*;

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub struct Req {
        pub to: Vec<sig::PublicKey>,
        pub msg: Bytes,
    }

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Success,
        Missing(Vec<sig::PublicKey>),
    }
}

pub mod new_key {
    use super::*;

    /// New endorsed key
    pub type Req = Signed<sig::Endorsement>;
    /// Result from trying to add key
    pub type Res = PKIResponse;
}

pub mod dep_key {
    use super::*;

    /// New deprecated key
    pub type Req = Signed<sig::Deprecation>;
    /// Result from trying to deprecate key
    pub type Res = PKIResponse;
}

pub mod new_prekey {
    use super::*;

    /// Signed prekey to be added
    pub type Req = Signed<Prekey>;
    /// Replaced prekey
    pub type Res = Prekey;
}

pub mod get_prekey {
    use super::*;

    /// Public key to fetch prekey for
    pub type Req = sig::PublicKey;
    /// Corresponding prekey
    pub type Res = Option<Signed<Prekey>>;
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum Request {
    KeysOf(keys_of::Req),
    KeyInfo(key_info::Req),
    UserExists(user_exists::Req),
    PushUsers(push_users::Req),
    PushDevices(push_devices::Req),
    NewKey(new_key::Req),
    DepKey(dep_key::Req),
    NewPrekey(new_prekey::Req),
    GetPrekey(get_prekey::Req),
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum Response {
    KeysOf(keys_of::Res),
    KeyInfo(key_info::Res),
    UserExists(user_exists::Res),
    PushUsers(push_users::Res),
    PushDevices(push_devices::Res),
    NewKey(new_key::Res),
    DepKey(dep_key::Res),
    NewPrekey(new_prekey::Res),
    GetPrekey(get_prekey::Res),
}
