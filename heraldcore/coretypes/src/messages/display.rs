use super::*;
use std::fmt;

impl fmt::Display for MessageBody {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for MissingInboundMessageField {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            MissingInboundMessageField::MissingMessageId => write!(f, "Message id was missing"),
            MissingInboundMessageField::MissingBody => write!(f, "Body was missing"),
            MissingInboundMessageField::MissingConversationId => {
                write!(f, "Conversation id was missing")
            }
            MissingInboundMessageField::MissingTimestamp => write!(f, "Timestamp was missing"),
            MissingInboundMessageField::MissingAuthor => write!(f, "Author was missing"),
        }
    }
}

impl fmt::Display for MissingOutboundMessageField {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            MissingOutboundMessageField::MissingBody => write!(f, "Body was missing"),
            MissingOutboundMessageField::MissingConversationId => {
                write!(f, "Conversation id was missing")
            }
        }
    }
}

impl fmt::Display for EmptyMessageBody {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Encountered empty message body: bodies must have at least one character"
        )
    }
}
