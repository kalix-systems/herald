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

macro_rules! imp_clone {
    ($($name: ident, $field: ident, $ret: ty),*) => {
       $(pub(super) fn $name(&self, index: usize) -> Option<$ret> {
            let id = &self.list.get(index).as_ref()?.id;
            Some(shared::conv_data().read().get(id)?.$field.clone())
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

    imp! {
        color_inner, color, u32,
        pairwise_inner, pairwise, bool,
        muted_inner, muted, bool,
        expiration_inner, expiration_period, ExpirationPeriod,
        status_inner, status, heraldcore::conversation::Status
    }

    imp_clone! {
        picture_inner, picture, Option<String>,
        title_inner, title, Option<String>
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
