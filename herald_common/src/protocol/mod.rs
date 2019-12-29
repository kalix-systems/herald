use super::*;
use krpc::*;

pub mod auth;
pub mod pushes;
pub mod requests;

pub struct HeraldProtocol {}
pub type PushAck = bool;

impl Protocol for HeraldProtocol {
    type Req = requests::Request;
    type Res = requests::Response;

    type Push = pushes::Push;
    type PushAck = PushAck;

    const MAX_CONCURRENT_REQS: usize = 10;
    const MAX_CONCURRENT_PUSHES: usize = 1;

    const MAX_REQ_SIZE: usize = u32::max_value() as usize;
    const MAX_ACK_SIZE: usize = 4;

    const MAX_RESP_SIZE: usize = u32::max_value() as usize;
    const MAX_PUSH_SIZE: usize = u32::max_value() as usize;
}
