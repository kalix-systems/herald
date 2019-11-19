use super::shared;
use super::*;

macro_rules! imp {
    ($($name: ident, $field: ident, $ret: ty),*) => {
       $(pub(super) fn $name(&self, index: usize) -> Option<$ret> {
            Some(self.data(index)?.$field)
       })*
    }
}

macro_rules! set_imp {
    ($($name: ident, $field: ident, $val: ty),*) => {
       $(pub(super) fn $name(&mut self, index: usize, val: $val) -> Option<()> {
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
    pub(super) fn id(&self, index: usize) -> Option<ConversationId> {
        Some(self.list.get(index)?.id)
    }

    fn data(&self, index: usize) -> Option<Ref> {
        let id = &self.list.get(index).as_ref()?.id;
        shared::data(id)
    }

    fn data_mut(&self, index: usize) -> Option<RefMut> {
        let id = &self.list.get(index).as_ref()?.id;
        shared::data_mut(id)
    }

    imp! {
        color_, color, u32,
        pairwise_, pairwise, bool,
        muted_, muted, bool,
        expiration_, expiration_period, ExpirationPeriod
    }

    imp_clone! {
        picture_, picture, Option<String>,
        title_, title, Option<String>
    }

    set_imp! {
        set_muted_, muted, bool,
        set_color_, color, u32,
        set_picture_, picture, Option<String>,
        set_expiration_, expiration_period, ExpirationPeriod,
        set_title_, title, Option<String>
    }
}

pub(super) fn init() {
    spawn!({
        let mut list = Vector::new();
        for meta in ret_err!(conversation::all_meta()).into_iter() {
            let (conv, data) = split_meta(meta);
            shared::insert_data(conv.id, data);
            list.push_back(conv);
        }

        ret_err!(Conversations::push(ConvUpdate::Init(list)));
    });
}
