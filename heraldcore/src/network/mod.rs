use crate::updates::Notification;
use crate::{errors::HErr, message::ReceiptStatus, pending, types::*, *};
use herald_common::*;
use herald_network as hn;
use std::{
    net::SocketAddr,
    sync::atomic::{AtomicBool, Ordering},
};
use websocket::{message::OwnedMessage as WMessage, sync::client as wsclient};

mod requester;

pub use registration::{begin_registration, finish_registration};
mod registration;

pub use login::login;
mod login;

use handlers::handle_push;
mod handlers;

mod statics;
use statics::*;
