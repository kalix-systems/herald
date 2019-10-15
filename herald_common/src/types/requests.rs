use super::*;

pub mod keys_of {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Vec<UserId>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub Vec<(UserId, UserMeta)>);
}

pub mod key_info {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Vec<sig::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub Vec<(sig::PublicKey, sig::PKMeta)>);
}

pub mod keys_exist {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Vec<sig::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub Vec<bool>);
}

pub mod users_exist {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Vec<UserId>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub Vec<bool>);
}

pub mod push_users {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req {
        pub to: Vec<UserId>,
        pub exc: sig::PublicKey,
        pub msg: Bytes,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Success,
        Missing(Vec<UserId>),
    }
}

pub mod push_devices {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req {
        pub to: Vec<sig::PublicKey>,
        pub msg: Bytes,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Success,
        Missing(Vec<sig::PublicKey>),
    }
}

pub mod new_key {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Signed<sig::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub PKIResponse);
}

pub mod dep_key {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Signed<sig::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Res(pub PKIResponse);
}

pub mod register {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Req(pub UserId, pub Signed<sign::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Res {
        UIDTaken,
        KeyTaken,
        BadSig(SigValid),
        Success,
    }
}

pub mod add_prekeys {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub Vec<sealed::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Missing(Vec<sig::PublicKey>),
        BadSig(SigValid),
        Success,
    }
}

pub mod get_prekeys {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Req(pub BTreeSet<sig::PublicKey>);

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Success(Vec<(sig::PublicKey, sealed::PublicKey)>),
        Missing(Vec<sig::PublicKey>),
    }
}
