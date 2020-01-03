use super::*;
use messages_helper::types::MessageBuilderHelper;

impl MessageBuilderHelper for MessageBuilder {
    fn try_clear_reply(
        &mut self,
        mid: &MsgId,
    ) {
        self.try_clear_reply(mid)
    }
}
