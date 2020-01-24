use crate::updates::Notification;
use crate::{errors::HErr, message::ReceiptStatus, pending, types::*, *};
use herald_common::*;
use herald_network as hn;
use hn::PushHandler;
use std::{
    net::SocketAddr,
    sync::atomic::{AtomicBool, Ordering},
};

mod requester;
pub use requester::*;

pub use registration::{begin_registration, finish_registration};
mod registration;

pub use login::login;
mod login;

use handlers::Pushy;
mod handlers;

mod statics;
use statics::*;

mod event;
use event::Event;
