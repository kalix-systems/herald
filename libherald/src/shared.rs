use heraldcore::errors::HErr;

/// A bus interface for pushing updates to singleton objects
pub trait SingletonBus {
    /// The type of the update message
    type Update: Send;

    /// Pushes the update through the bus
    fn push(update: Self::Update) -> Result<(), HErr>;
}

/// A bus interface for pushing updates to dynamically created objects
pub trait AddressedBus {
    /// The type of the update message
    type Update: Send;

    /// The type used to address the objects
    type Addr: std::hash::Hash;

    /// Pushes the update through the bus, to the object addressed by `to`
    fn push(
        to: Self::Addr,
        update: Self::Update,
    ) -> Result<(), HErr>;
}
