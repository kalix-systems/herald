use super::*;

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum Recips {
    Users(Vec<UserId>),
    Keys(Vec<sig::PublicKey>),
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum SingleRecip {
    User(UserId),
    Key(sig::PublicKey),
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub enum Recip {
    One(SingleRecip),
    Many(Recips),
}

impl Recip {
    pub fn tag(&self) -> PushTag {
        match self {
            Recip::One(SingleRecip::User(_)) => PushTag::User,
            Recip::One(SingleRecip::Key(_)) => PushTag::Key,
            Recip::Many(Recips::Users(_)) => PushTag::User,
            Recip::Many(Recips::Keys(_)) => PushTag::Key,
        }
    }
}

pub mod get_sigchain {
    use super::*;

    /// [`UserId`] to fetch keys of
    pub type Req = UserId;

    /// [`UserMeta`] found for requested [`UserId`], `None` where the user was not found.
    pub type Res = Option<sig::SigChain>;
}

pub mod recip_exists {
    use super::*;

    /// [`Recip`] to check existence of
    pub type Req = Recip;

    /// `true` if requested [`Recip`] exists, false otherwise
    pub type Res = bool;
}

pub mod new_sig {
    use super::*;

    pub type Req = Signed<sig::SigUpdate>;
    pub type Res = PKIResponse;
}

pub mod new_prekeys {
    use super::*;

    pub type Req = Vec<(Signed<Prekey>, Option<Prekey>)>;

    #[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Res {
        Success,
        BadSig(SigValid, Prekey),
        Redundant(Prekey),
        DeadKey(Prekey),
        NoSlotAvailable(Prekey),
    }
}

pub mod get_prekeys {
    use super::*;

    /// Public key to fetch prekeys for
    pub type Req = Vec<sig::PublicKey>;

    /// Corresponding prekeys
    pub type Res = Vec<(sig::PublicKey, Signed<Prekey>)>;
}

pub mod add_to_group {
    use super::*;

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub struct Req {
        users: Vec<UserId>,
        conversation: ConversationId,
    }

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Success,
        AddedByMissing(UserId),
        MissingUser(UserId),
    }
}

pub mod init_group {
    use super::*;

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Success,
        GroupAlreadyExists(ConversationId),
        MissingUser(UserId),
    }
}

pub mod leave_groups {
    use super::*;

    pub type Req = Vec<ConversationId>;

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Success,
        Missing(ConversationId),
    }
}

pub mod push {
    use super::*;

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub struct Req {
        pub to: Recip,
        pub msg: Bytes,
    }

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub enum Res {
        Success,
        Missing(SingleRecip),
    }
}

macro_rules! proto_enum {
    ($name:ident, $inner:ident, [ $($extra:tt)* ]) => {
        #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
        pub enum $name {
            GetSigchain(get_sigchain::$inner),
            RecipExists(recip_exists::$inner),

            NewSig(new_sig::$inner),

            NewPrekey(new_prekeys::$inner),
            GetPrekey(get_prekeys::$inner),

            // AddToGroup(add_to_group::$inner),
            // LeaveGroups(leave_groups::$inner),

            Push(push::$inner),

            $($extra)*
        }
    };
    ($name:ident, $inner:ident) => {
        proto_enum!($name,$inner,[]);
    };
}

proto_enum!(Request, Req);
proto_enum!(Response, Res, [Err(String)]);
