// extern crate herald_common;
// extern crate heraldcore;

// use herald_common::UserId;
// use heraldcore::db::reset_all;
// use heraldcore::network::{self, Notification};
// use serial_test_derive::serial;
// use std::convert::TryFrom;
// use std::process::Command;
// use std::thread::sleep;
// use std::time::Duration;

// fn server_initialization() -> std::process::Child {
//     let mut child = Command::new("diesel")
//         .arg("migration")
//         .arg("redo")
//         .current_dir("../server")
//         .spawn()
//         .expect("failed to execute child");
//     let ecode = child.wait().expect("failed to wait on child");
//     assert!(ecode.success());

//     let mut child = Command::new("cargo")
//         .arg("build")
//         .current_dir("../server")
//         .spawn()
//         .expect("failed to execute child");
//     let ecode = child.wait().expect("failed to wait on child");
//     assert!(ecode.success());

//     Command::new("cargo")
//         .arg("run")
//         .current_dir("../server")
//         .spawn()
//         .expect("could not spawn server")
// }

// fn assert_success(notif: Notification) {}

// #[test]
// #[serial]
// fn connects_to_server() {
//     let reset_res = reset_all();
//     assert!(reset_res.is_ok());

//     let mut server_process = server_initialization();
//     std::thread::sleep(Duration::from_secs(2));

//     let bob = UserId::try_from("Bob").expect("could not make bob");

//     if !network::register(bob).is_ok() {
//         server_process.kill().expect("server crashed during test");
//         panic!("Could not register new user")
//     }

//     let login_res = network::login(assert_success);

//     if !login_res.is_ok() {
//         server_process.kill().expect("server crashed during test");
//         panic!("Could not register new user")
//     }

//     server_process.kill().expect("server crashed during test");
// }
