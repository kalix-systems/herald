use super::{shared, *};
use crate::push;

impl Conversations {
    pub(crate) fn id(
        &self,
        index: usize,
    ) -> Option<ConversationId> {
        Some(self.list.get(index)?.id)
    }
}

macro_rules! imp {
    ($($name: ident, $field: ident, $ret: ty),*) => {
       $(pub(crate) fn $name(&self, index: usize) -> Option<$ret> {
            let id = &self.list.get(index).as_ref()?.id;
            Some(shared::conv_data().read().get(id)?.$field)
       })*
    }
}

macro_rules! set_imp {
    ($($name: ident, $field: ident, $val: ty),*) => {
       $(pub(crate) fn $name(&mut self, index: usize, val: $val) -> Option<()> {
            let id = &self.list.get(index).as_ref()?.id;
            let mut lock = shared::conv_data().write();
            let data = lock.get_mut(id)?;

            data.$field = val;
            Some(())
       })*
    }
}

impl Conversations {
    pub(crate) fn inner_filter(&mut self) -> Option<()> {
        let filter = &self.filter.as_ref()?;

        let list = &mut self.list;
        for (ix, Conversation { matched, id }) in list.iter_mut().enumerate() {
            let lock = shared::conv_data().read();
            let data = cont_none!(lock.get(id));

            let new_matched = match &data.title {
                Some(title) => filter.is_match(&title),
                None => false,
            };

            if new_matched != *matched {
                *matched = new_matched;
                self.model.data_changed(ix, ix);
            }
        }

        Some(())
    }

    pub(crate) fn is_empty_(
        &self,
        index: usize,
    ) -> Option<bool> {
        let cid = &self.list.get(index).as_ref()?.id;
        shared::last_msg_id(&cid).is_none().into()
    }

    pub(crate) fn last_msg_digest_(
        &self,
        index: usize,
    ) -> Option<String> {
        use heraldcore::message::MsgData;

        let cid = &self.list.get(index).as_ref()?.id;
        let mid = shared::last_msg_id(&cid)?;

        let MsgData {
            author,
            receipts,
            send_status,
            time,
            content,
            ..
        } = messages_helper::container::get(&mid)?;

        let body = content.as_str();
        let time = time.insertion;
        let aux_code = content.aux_code();
        let has_attachments = content
            .attachments()
            .map(|a| !a.is_empty())
            .unwrap_or(false);

        let receipt_status = receipts.iter().map(|(_, r)| r).max().copied();
        let status = heraldcore::message::Status::from((send_status, receipt_status)) as u8;

        let object = json::object! {
            "author" => author.as_str(),
            "body" => body,
            "time" => *time.as_i64(),
            "auxCode" => aux_code,
            "status" => status,
            "hasAttachments" => has_attachments
        };

        Some(object.dump())
    }

    pub(crate) fn pairwise_inner(
        &self,
        index: usize,
    ) -> Option<bool> {
        let id = &self.list.get(index).as_ref()?.id;
        shared::pairwise(id)
    }

    pub(crate) fn color_inner(
        &self,
        index: usize,
    ) -> Option<u32> {
        let id = &self.list.get(index).as_ref()?.id;
        let read = shared::conv_data().read();
        let data = read.get(id)?;

        match data.pairwise_uid {
            Some(uid) => crate::users::shared::color(&uid),
            None => data.color.into(),
        }
    }

    pub(crate) fn title_inner(
        &self,
        index: usize,
    ) -> Option<String> {
        let id = &self.list.get(index).as_ref()?.id;
        let read = shared::conv_data().read();
        let data = read.get(id)?;

        match data.pairwise_uid {
            Some(uid) => crate::users::shared::name(&uid),
            None => data.title.clone(),
        }
    }

    pub(crate) fn picture_inner(
        &self,
        index: usize,
    ) -> Option<String> {
        let id = &self.list.get(index).as_ref()?.id;
        let read = shared::conv_data().read();
        let data = read.get(id)?;

        match data.pairwise_uid {
            Some(uid) => crate::users::shared::profile_picture(&uid),
            None => data.picture.clone(),
        }
    }

    imp! {
        muted_inner, muted, bool,
        expiration_inner, expiration_period, ExpirationPeriod,
        status_inner, status, heraldcore::conversation::Status
    }

    set_imp! {
        set_muted_inner, muted, bool,
        set_picture_inner, picture, Option<String>,
        set_expiration_inner, expiration_period, ExpirationPeriod,
        set_title_inner, title, Option<String>,
        set_status_inner, status, heraldcore::conversation::Status
    }
}

impl crate::Loadable for Conversations {
    type Error = std::io::Error;

    fn try_load(&mut self) -> Result<(), std::io::Error> {
        std::thread::Builder::new().spawn(|| {
            let mut list = Vector::new();
            for meta in err!(conversation::all_meta()).into_iter() {
                let (conv, data) = split_meta(meta);
                shared::insert_data(conv.id, data);
                list.push_back(conv);
            }

            push(GlobalConvUpdate::Init(list));
        })?;

        Ok(())
    }

    fn loaded(&self) -> bool {
        self.loaded
    }
}
