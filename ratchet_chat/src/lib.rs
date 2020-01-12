pub mod protocol;
pub mod ratchet;

pub trait StoreLike {
    type Error: std::error::Error + Send + 'static;
}
