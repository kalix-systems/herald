use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Serialize, Deserialize)]
pub struct Instruction {
    // the type of action that will be executed
    pub action_type: Option<String>,
    // what to await for, if the action type is an await
    pub what: Option<String>,
    // UserId string
    pub from: Option<String>,
    // number of ms to wait before failing
    pub timeout: Option<i64>,
    // who to send the message to, if it is a message
    pub to: Option<String>,
    // the body of the message, if it has one
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ScriptFile {
    pub userid: Option<String>,
    pub actions: Vec<Instruction>,
}

impl From<Vec<u8>> for ScriptFile {
    fn from(buf: Vec<u8>) -> ScriptFile {
        let json_string = String::from_utf8(buf).expect("JSON file was not text.");
        let script = serde_json::from_str(&json_string).expect("Invalid JSON.");
        script
    }
}

impl ScriptFile {
    pub fn new(buf: Vec<u8>) -> Self {
        buf.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes() {
        let test_string = r#"
            {
                "userid" : "hello",
                "actions" : [
                 {
                 "action_type": "Send",
                 "to": "Alice",
                 "body": "Hello Alice"
                 }
                ]
            }"#;

        let script: ScriptFile = ScriptFile::new(test_string.into());

        assert_eq!(&script.userid.unwrap(), "hello");
        assert_eq!(script.actions[0].action_type.as_ref().unwrap(), "Send");
        assert_eq!(script.actions[0].to.as_ref().unwrap(), "Alice");
        assert_eq!(script.actions[0].body.as_ref().unwrap(), "Hello Alice");
    }
}
