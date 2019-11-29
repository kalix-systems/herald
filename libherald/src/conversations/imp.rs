use super::{shared, *};
use crate::push;

impl Conversations {
    pub(crate) fn id(
        &self,
        index: usize,
    ) -> Option<ConversationId> {
        Some(self.list.get(index)?.id)
    }

    pub(crate) fn data(
        &self,
        index: usize,
    ) -> Option<Ref> {
        let id = &self.list.get(index).as_ref()?.id;
        shared::data(id)
    }

    pub(crate) fn data_mut(
        &self,
        index: usize,
    ) -> Option<RefMut> {
        let id = &self.list.get(index).as_ref()?.id;
        shared::data_mut(id)
    }
}

macro_rules! imp {
    ($($name: ident, $field: ident, $ret: ty),*) => {
       $(pub(crate) fn $name(&self, index: usize) -> Option<$ret> {
            Some(self.data(index)?.$field)
       })*
    }
}

macro_rules! set_imp {
    ($($name: ident, $field: ident, $val: ty),*) => {
       $(pub(crate) fn $name(&mut self, index: usize, val: $val) -> Option<()> {
            let mut data = self.data_mut(index)?;
            data.$field = val;
            Some(())
       })*
    }
}

macro_rules! imp_clone {
    ($($name: ident, $field: ident, $ret: ty),*) => {
       $(pub(super) fn $name(&self, index: usize) -> Option<$ret> {
            Some(self.data(index)?.$field.clone())
       })*
    }
}

impl Conversations {
    pub(crate) fn inner_filter(&mut self) -> Option<()> {
        let filter = &self.filter.as_ref()?;

        let list = &mut self.list;
        for (ix, Conversation { matched, id }) in list.iter_mut().enumerate() {
            let data = cont_none!(shared::data(id));

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
        expiration_inner, expiration_period, ExpirationPeriod
    }

    imp_clone! {
        picture_inner, picture, Option<String>,
        title_inner, title, Option<String>
    }

    set_imp! {
        set_muted_inner, muted, bool,
        set_color_inner, color, u32,
        set_picture_inner, picture, Option<String>,
        set_expiration_inner, expiration_period, ExpirationPeriod,
        set_title_inner, title, Option<String>
    }
}

impl crate::Loadable for Conversations {
    type Error = std::io::Error;

    fn try_load(&mut self) -> Result<(), std::io::Error> {
        std::thread::Builder::new().spawn(|| {
            let mut list = Vector::new();
            for meta in ret_err!(conversation::all_meta()).into_iter() {
                let (conv, data) = split_meta(meta);
                shared::insert_data(conv.id, data);
                list.push_back(conv);
            }

            ret_err!(push(ConvUpdate::Init(list)));
        })?;

        Ok(())
    }

    fn loaded(&self) -> bool {
        self.loaded
    }
}
