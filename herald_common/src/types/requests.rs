use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RequestError {
    UnknownError,
    DbError(String),
}

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

#[tarpc::service]
trait HeraldService {
    async fn keys_of(req: keys_of::Req) -> Result<keys_of::Res, RequestError>;
    async fn key_info(req: key_info::Req) -> Result<key_info::Res, RequestError>;
    async fn keys_exist(req: keys_exist::Req) -> Result<keys_exist::Res, RequestError>;
    async fn users_exist(req: users_exist::Req) -> Result<users_exist::Res, RequestError>;
    async fn push_users(req: push_users::Req) -> Result<push_users::Res, RequestError>;
    async fn push_devices(req: push_devices::Req) -> Result<push_devices::Res, RequestError>;
    async fn new_key(req: new_key::Req) -> Result<new_key::Res, RequestError>;
    async fn dep_key(req: dep_key::Req) -> Result<dep_key::Res, RequestError>;
    async fn register(req: register::Req) -> Result<register::Res, RequestError>;
    async fn add_prekeys(req: add_prekeys::Req) -> Result<add_prekeys::Res, RequestError>;
    async fn get_prekeys(req: get_prekeys::Req) -> Result<get_prekeys::Res, RequestError>;
}

pub mod transport {
    pub use tarpc_bincode_transport::*;
}
