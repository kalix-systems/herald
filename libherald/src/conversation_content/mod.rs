use crate::{
    conversations::shared as cs,
    err, ffi,
    imp::{Members, Messages},
    interface::{ConversationContentEmitter as Emitter, ConversationContentTrait as Interface},
    members::MemberUpdate,
    none, spawn,
};
use heraldcore::types::ConversationId;
use std::convert::TryFrom;

mod shared;
pub(crate) use shared::{content_push, new_activity};

/// Wrapper around `Messages` and `Members`
pub struct ConversationContent {
    emit: Emitter,
    members: Members,
    messages: Messages,
    id: Option<ConversationId>,
}

impl Interface for ConversationContent {
    fn new(
        emit: Emitter,
        members: Members,
        messages: Messages,
    ) -> Self {
        Self {
            emit,
            members,
            messages,
            id: None,
        }
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        self.id.as_ref().map(ConversationId::as_slice)
    }

    fn set_conversation_id(
        &mut self,
        cid: Option<ffi::ConversationIdRef>,
    ) {
        if let (Some(id), None) = (cid, self.id) {
            let id = err!(ConversationId::try_from(id));

            self.id.replace(id);
            self.emit.conversation_id_changed();
            none!(self.register_model());

            self.messages.set_conversation_id(id);
            err!(self.members.set_conversation_id(id));
        }
    }

    fn poll_update(&mut self) {
        none!(self.process_updates());
    }

    fn members(&self) -> &Members {
        &self.members
    }

    fn members_mut(&mut self) -> &mut Members {
        &mut self.members
    }

    fn messages(&self) -> &Messages {
        &self.messages
    }

    fn messages_mut(&mut self) -> &mut Messages {
        &mut self.messages
    }

    fn conversation_color(&self) -> u32 {
        let f = || {
            let read = cs::conv_data().read();
            let data = read.get(&self.id?)?;
            match data.pairwise_uid {
                Some(uid) => crate::users::shared::color(&uid),
                None => data.color.into(),
            }
        };

        f().unwrap_or(0)
    }

    fn expiration_period(&self) -> u8 {
        let f = || Some(cs::conv_data().read().get(&self.id?)?.expiration_period);

        f().unwrap_or_default() as u8
    }

    fn set_expiration_period(
        &mut self,
        period: u8,
    ) {
        let period = period.into();

        let f = || {
            let id = &self.id?;
            let mut lock = cs::conv_data().write();
            let data = lock.get_mut(id)?;
            data.expiration_period = period;
            Some(*id)
        };

        let id = none!(f());

        spawn!({
            err!(heraldcore::conversation::set_expiration_period(&id, period));
        });

        self.emit.expiration_period_changed();
    }

    fn muted(&self) -> bool {
        let f = || Some(cs::conv_data().read().get(&self.id?)?.muted);

        f().unwrap_or(false)
    }

    fn set_muted(
        &mut self,
        muted: bool,
    ) {
        let f = || {
            let id = self.id?;
            spawn!(err!(heraldcore::conversation::set_muted(&id, muted)), None);

            let mut lock = cs::conv_data().write();
            let data = lock.get_mut(&id)?;

            data.muted = muted;

            Some(())
        };

        none!(f());

        self.emit.muted_changed();
    }

    fn pairwise(&self) -> bool {
        self.id.as_ref().and_then(cs::pairwise).unwrap_or(false)
    }

    fn picture(&self) -> Option<String> {
        self.id.as_ref().and_then(cs::picture)
    }

    fn status(&self) -> u8 {
        let f = || Some(cs::conv_data().read().get(&self.id?)?.status);
        f().unwrap_or_default() as u8
    }

    fn set_status(
        &mut self,
        status: u8,
    ) {
        use heraldcore::conversation::{self, Status};

        let f = || {
            let id = self.id?;
            let status = Status::from_u8(status)?;

            let mut lock = cs::conv_data().write();
            let data = lock.get_mut(&id)?;

            data.status = status;

            spawn!(err!(conversation::set_status(&id, status)), None);

            Some(())
        };

        none!(f());

        self.emit.status_changed();
    }

    fn title(&self) -> Option<String> {
        self.id.as_ref().and_then(cs::title)
    }

    fn set_title(
        &mut self,
        title: Option<String>,
    ) {
        let f = || {
            let id = self.id?;
            {
                let title = title.clone();
                spawn!(err!(heraldcore::conversation::set_title(&id, title)), None);
            }

            let mut lock = cs::conv_data().write();
            let data = lock.get_mut(&id)?;

            data.title = title;
            Some(())
        };

        none!(f());

        self.emit.title_changed();
    }

    fn set_picture(
        &mut self,
        picture_json: String,
    ) {
        let cid = none!(self.id);

        let profile_picture =
            heraldcore::image_utils::ProfilePicture::from_json_string(picture_json);

        let mut emit = self.emit.clone();

        spawn!({
            let path = err!(heraldcore::conversation::set_picture(&cid, profile_picture));
            let mut write = cs::conv_data().write();
            if let Some(data) = write.get_mut(&cid) {
                data.picture = path;
                emit.picture_changed();
            }
        });
    }
}
