use crate::interface::*;

type Emitter = ConversationBuilderEmitter;

/// A builder for conversations
pub struct ConversationBuilder {
    emit: Emitter,
}

// TODO fill in stubs
#[allow(unused)]
impl ConversationBuilderTrait for ConversationBuilder {
    fn new(emit: Emitter) -> Self {
        Self { emit }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn add_user(&mut self, user_id: String) -> bool {
        false
    }

    fn finalize(&mut self) -> Vec<u8> {
        vec![]
    }

    fn set_color(&mut self, color: u32) -> bool {
        false
    }

    fn set_picture(&mut self, picture_path: String) -> bool {
        false
    }

    fn set_title(&mut self, title: String) -> bool {
        false
    }
}
