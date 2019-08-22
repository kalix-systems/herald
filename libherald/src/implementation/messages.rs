use crate::interface::*;
use heraldcore::{
    db::DBTable,
    message::{Message, Messages as Core},
};
use im_rc::vector::Vector as ImVector;

#[derive(Clone)]
struct MessagesItem {
    message_id: i64,
    author: String,
    recipient: String,
    body: String,
    timestamp: String,
}

impl From<Message> for MessagesItem {
    #[inline]
    fn from(m: Message) -> MessagesItem {
        let Message {
            message_id,
            author,
            recipient,
            body,
            timestamp,
        } = m;
        MessagesItem {
            message_id,
            author,
            recipient,
            body,
            timestamp: timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

pub struct Messages {
    conversation_id: Option<String>,
    emit: MessagesEmitter,
    model: MessagesList,
    list: ImVector<MessagesItem>,
}

impl MessagesTrait for Messages {
    fn new(emit: MessagesEmitter, model: MessagesList) -> Messages {
        if let Err(e) = Core::create_table() {
            eprintln!("{}", e);
        }
        Messages {
            conversation_id: None,
            list: ImVector::new(),
            model,
            emit,
        }
    }

    fn set_conversation_id(&mut self, conversation_id: Option<String>) {
        println!("Setting conversation_id to: {:?}", conversation_id);
        self.conversation_id = conversation_id;

        if let Some(conversation_id) = self.conversation_id.as_ref() {
            self.model.begin_reset_model();
            self.list = ImVector::new();
            self.model.end_reset_model();

            let messages: ImVector<MessagesItem> =
                match Core::get_conversation(conversation_id.as_str()) {
                    Ok(ms) => ms.into_iter().map(|m| m.into()).collect(),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        return;
                    }
                };

            if messages.is_empty() {
                return;
            }

            self.model.begin_insert_rows(0, messages.len() - 1);
            self.list = messages;
            self.model.end_insert_rows();
            println!(
                "Inserted {} messages with {}",
                self.list.len(),
                conversation_id
            )
        }
    }

    fn author(&self, row_index: usize) -> &str {
        self.list[row_index].author.as_str()
    }

    fn recipient(&self, row_index: usize) -> &str {
        self.list[row_index].recipient.as_str()
    }

    fn body(&self, row_index: usize) -> &str {
        self.list[row_index].body.as_str()
    }

    fn message_id(&self, row_index: usize) -> i64 {
        self.list[row_index].message_id
    }

    fn conversation_id(&self) -> Option<&str> {
        self.conversation_id.as_ref().map(|s| s.as_str())
    }

    // TODO add networking component
    fn send_message(&mut self, body: String) -> bool {
        let id = match heraldcore::config::Config::static_id() {
            Ok(id) => id,
            Err(e) => {
                eprintln!("{}", e);
                return false;
            }
        };

        let conversation_id = match &self.conversation_id {
            Some(conv) => conv,
            None => {
                eprintln!("Error: conversation_id not set.");
                return false;
            }
        };

        match Core::add_message(id.as_str(), conversation_id.as_str(), body.as_str(), None) {
            Ok((msg_id, timestamp)) => {
                let msg = MessagesItem {
                    author: id,
                    recipient: self.conversation_id.clone().unwrap_or("userid2".into()),
                    body: body,
                    message_id: msg_id,
                    timestamp,
                };
                self.model
                    .begin_insert_rows(self.row_count(), self.row_count());
                self.list.push_back(msg);
                self.model.end_insert_rows();
                true
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                false
            }
        }
    }

    fn delete_message(&mut self, row_index: u64) -> bool {
        let row_index = row_index as usize;
        let id = self.list[row_index].message_id;
        match Core::delete_message(id) {
            Ok(_) => {
                self.model.begin_remove_rows(row_index, row_index);
                self.list.remove(row_index);
                self.model.end_remove_rows();
                true
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                false
            }
        }
    }

    fn emit(&mut self) -> &mut MessagesEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }
}
