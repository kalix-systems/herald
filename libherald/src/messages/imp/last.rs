use super::*;

impl Messages {
    pub(crate) fn last_author_(&self) -> Option<ffi::UserIdRef> {
        let last = self.container.last_msg()?;

        if last.author == self.local_id? {
            Some("You")
        } else {
            Some(last.author.as_str())
        }
    }

    pub(crate) fn last_body_(&self) -> Option<&str> {
        Some(self.container.last_msg()?.body.as_ref()?.as_str())
    }

    pub(crate) fn last_time_(&self) -> Option<i64> {
        Some(self.container.last_msg()?.time.insertion.into())
    }
}
