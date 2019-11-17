use rust_qt_binding_generator::configuration::SimpleType::*;
use rust_qt_binding_generator::configuration::*;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

pub mod func;
pub mod item_prop;
pub mod macros;
pub mod obj;
pub mod prop;

use func::*;
use item_prop::*;
use obj::*;
use prop::*;

use crate::*;

pub(crate) fn get() -> Config {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let parent_dir = crate_dir.parent().unwrap();

    let cpp_file = PathBuf::from("qt_ffi/Bindings.cpp");

    let objects = objects();
    let rust = Rust {
        dir: parent_dir.to_path_buf(),
        implementation_module: "imp".into(),
        interface_module: "interface".into(),
    };
    let rust_edition = RustEdition::Rust2018;
    let overwrite_implementation = false;

    Config {
        out_dir: parent_dir.to_path_buf(),
        rust_edition,
        cpp_file,
        overwrite_implementation,
        rust,
        objects,
    }
}

fn objects() -> BTreeMap<String, Rc<Object>> {
    objects! {
       herald_state(),
       errors(),
       herald_utils(),
       conversations(),
       users(),
       members(),
       messages(),
       message_preview(),
       config_obj(),
       conversation_builder(),
       conversation_builder_users(),
       message_builder(),
       attachments(),
       global_message_search()
    }
}

fn herald_state() -> Object {
    let properties = props! {
        configInit: Prop::new().simple(Bool),
        connectionUp: Prop::new().simple(Bool),
        connectionPending: Prop::new().simple(Bool)
    };

    let funcs = functions! {
        mut registerNewUser(user_id: QString) => Void,
        mut login() => Bool,
    };

    obj! {
        HeraldState: Obj::new().props(properties).funcs(funcs)
    }
}

fn errors() -> Object {
    let properties = props! {
        tryPoll:  Prop::new().simple(QUint8)
    };

    let functions = functions! {
        mut nextError() => QString,
    };

    obj! {
        Errors: Obj::new().props(properties).funcs(functions)
    }
}

fn herald_utils() -> Object {
    let functions = functions! {
        const compareByteArray(bs1: QByteArray, bs2: QByteArray) => Bool,
        const isValidRandId(bs: QByteArray) => Bool,
    };

    obj! {
        HeraldUtils: Obj::new().funcs(functions)
    }
}

fn conv_id_prop() -> Prop {
    Prop::new()
        .simple(SimpleType::QByteArray)
        .write()
        .optional()
}

fn filter_prop() -> Prop {
    Prop::new().simple(SimpleType::QString).write()
}

fn filter_regex_prop() -> Prop {
    Prop::new().simple(SimpleType::Bool).write()
}

fn matched_item_prop() -> ItemProp {
    ItemProp::new(SimpleType::Bool)
}

fn filter_props() -> BTreeMap<String, Property> {
    props! {
        filter: filter_prop(),
        filterRegex: filter_regex_prop()
    }
}

fn color_item_prop() -> ItemProp {
    ItemProp::new(SimpleType::QUint32)
}

fn picture_item_prop() -> ItemProp {
    ItemProp::new(SimpleType::QString).optional()
}

fn conversations() -> Object {
    let props = filter_props();

    let item_props = item_props! {
       conversationId: ItemProp::new(QByteArray),
       title: ItemProp::new(QString).write().optional(),
       muted: ItemProp::new(Bool).write(),
       pairwise: ItemProp::new(Bool),
       expirationPeriod: ItemProp::new(QUint8).write(),
       matched: matched_item_prop(),
       picture: picture_item_prop().write(),
       color: color_item_prop().write()
    };

    let funcs = functions! {
        mut removeConversation(row_index: QUint64) => Bool,
        mut toggleFilterRegex() => Bool,
        mut clearFilter() => Void,
    };

    obj! {
       Conversations: Obj::new().list().props(props).item_props(item_props).funcs(funcs)
    }
}

fn users() -> Object {
    let props = filter_props();

    let item_props = item_props! {
       userId: ItemProp::new(QString),
       name: ItemProp::new(QString).get_by_value().write(),
       pairwiseConversationId: ItemProp::new(QByteArray).get_by_value(),
       status: ItemProp::new(QUint8).write(),
       matched: matched_item_prop(),
       profilePicture: picture_item_prop().get_by_value().write(),
       color: color_item_prop().write()
    };

    let funcs = functions! {
        mut add(id: QString) => QByteArray,
        mut toggleFilterRegex() => Bool,
        mut clearFilter() => Void,
        const colorById(id: QString) => QUint32,
        const nameById(id: QString) => QString,
        const profilePictureById(id: QString) => QString,
    };

    obj! {
        Users: Obj::new().list().props(props).funcs(funcs).item_props(item_props)
    }
}

fn members() -> Object {
    let mut props = props! {
        conversationId: conv_id_prop()
    };

    props.append(&mut filter_props());

    let item_props = item_props! {
       userId: ItemProp::new(QString),
       name: ItemProp::new(QString).get_by_value(),
       pairwiseConversationId: ItemProp::new(QByteArray).get_by_value(),
       status: ItemProp::new(QUint8),
       matched: matched_item_prop(),
       profilePicture: picture_item_prop().get_by_value(),
       color: color_item_prop()
    };

    let funcs = functions! {
        mut addToConversation(id: QString) => Bool,
        mut removeFromConversationByIndex(row_index: QUint64) => Bool,
        mut toggleFilterRegex() => Bool,
    };

    obj! {
        Members: Obj::new().list().props(props).funcs(funcs).item_props(item_props)
    }
}

fn message_preview() -> Object {
    let props = props! {
         messageId: Prop::new().simple(QByteArray).optional().write(),
         author: Prop::new().simple(QString).optional(),
         body: Prop::new().simple(QString).optional(),
         epochTimestampMs: Prop::new().simple(Qint64).optional(),
         isDangling: Prop::new().simple(Bool),
         hasAttachments: Prop::new().simple(Bool),
         msgIdSet: Prop::new().simple(Bool)
    };

    obj! {
       MessagePreview: Obj::new().props(props)
    }
}

fn messages() -> Object {
    let props = props! {
        conversationId: conv_id_prop(),
        lastAuthor: Prop::new().simple(QString).optional(),
        lastBody: Prop::new().simple(QString).optional(),
        lastEpochTimestampMs: Prop::new().simple(Qint64).optional(),
        lastStatus: Prop::new().simple(QUint32).optional(),
        isEmpty: Prop::new().simple(Bool),
        searchPattern: filter_prop(),
        searchRegex: filter_regex_prop(),
        searchActive: Prop::new().simple(Bool).write(),
        searchNumMatches: Prop::new().simple(QUint64)
    };

    let item_props = item_props! {
        messageId: ItemProp::new(QByteArray).optional(),
        author: ItemProp::new(QString).optional(),
        body: ItemProp::new(QString).optional(),
        epochTimestampMs: ItemProp::new(Qint64).optional(),
        serverTimestampMs: ItemProp::new(Qint64).optional(),
        expirationTimestampMs: ItemProp::new(Qint64).optional(),
        op: ItemProp::new(QByteArray).optional(),
        isReply: ItemProp::new(Bool).optional(),
        hasAttachments: ItemProp::new(Bool).optional(),
        receiptStatus: ItemProp::new(QUint32).optional(),
        dataSaved: ItemProp::new(Bool).optional(),
        isHead: ItemProp::new(Bool).optional(),
        isTail: ItemProp::new(Bool).optional(),
        // 0 => Not matched,
        // 1 => Matched,
        // 2 => Matched and selected
        match_status: ItemProp::new(QUint8).optional()
    };

    let funcs = functions! {
        mut deleteMessage(row_index: QUint64) => Bool,
        mut clearConversationHistory() => Bool,
        mut clearSearch() => Void,
        mut nextSearchMatch() => Qint64,
        mut prevSearchMatch() => Qint64,
        const indexById(msg_id: QByteArray) => QUint64,
    };

    obj! {
        Messages: Obj::new().list().funcs(funcs).item_props(item_props).props(props)
    }
}

fn config_obj() -> Object {
    let props = props! {
        configId: Prop::new().simple(QString),
        name: Prop::new().simple(QString).write(),
        profilePicture: Prop::new().simple(QString).write().optional(),
        color: Prop::new().simple(QUint32).write(),
        colorscheme: Prop::new().simple(QUint32).write(),
        ntsConversationId: Prop::new().simple(QByteArray)
    };

    obj! {
        Config: Obj::new().props(props)
    }
}

fn conversation_builder() -> Object {
    let item_prop = item_props! {
        memberId: ItemProp::new(QString)
    };

    let prop = props! {
        picture: Prop::new().simple(QString).write().optional()
    };

    let funcs = functions! {
        mut addMember(user_id: QString) => Bool,
        mut removeMemberById(user_id: QString) => Bool,
        mut removeMemberByIndex(index: QUint64) => Bool,
        mut removeLast() => Void,
        mut setTitle(title: QString) => Void,
        mut finalize() => Void,
    };

    obj! {
        ConversationBuilder: Obj::new().list().funcs(funcs).item_props(item_prop).props(prop)
    }
}

fn conversation_builder_users() -> Object {
    let props = props! {
        filter: Prop::new().simple(SimpleType::QString).write().optional()
    };

    let item_props = item_props! {
       userId: ItemProp::new(QString).optional(),
       name: ItemProp::new(QString).get_by_value().optional(),
       profilePicture: picture_item_prop().get_by_value().optional(),
       color: color_item_prop().optional(),
       selected: ItemProp::new(Bool).write(),
       matched: matched_item_prop()
    };

    let funcs = functions! {
        mut clearFilter() => Void,
    };

    obj! {
        ConversationBuilderUsers: Obj::new().list().props(props).funcs(funcs).item_props(item_props)
    }
}

fn message_builder() -> Object {
    let props = props! {
        // Conversation id
        conversationId: conv_id_prop(),
        // Message id of the message being replied to
        replyingTo: Prop::new().simple(QByteArray).optional().write(),
        isReply: Prop::new().simple(Bool),
        // Body of the message
        body: Prop::new().simple(QString).optional().write(),
        isMediaMessage: Prop::new().simple(Bool),
        parseMarkdown: Prop::new().simple(Bool).write()
    };

    let item_props = item_props! {
        attachmentPath: ItemProp::new(QString)
    };

    let funcs = functions! {
        mut finalize() => Void,
        mut clearReply() => Void,
        mut addAttachment(path: QString) => Bool,
        mut removeAttachment(path: QString) => Bool,
        mut removeAttachmentByIndex(row_index: QUint64) => Bool,
        mut removeLast() => Void,
    };

    obj! {
        MessageBuilder: Obj::new().list().funcs(funcs).item_props(item_props).props(props)
    }
}

fn attachments() -> Object {
    let props = props! {
        // the message id the attachments list is associated with
        msgId: Prop::new().simple(QByteArray).optional().write()
    };

    let item_props = item_props! {
        // Path the the attachment
        attachmentPath: ItemProp::new(QString)
    };

    obj! {
        Attachments: Obj::new().list().props(props).item_props(item_props)
    }
}

fn global_message_search() -> Object {
    let props = props! {
        searchPattern: Prop::new().simple(QString).optional().write(),
        regexSearch: Prop::new().simple(Bool).optional().write()
    };

    let item_props = item_props! {
        msgId: ItemProp::new(QByteArray).optional(),
        author: ItemProp::new(QString).optional(),
        conversation: ItemProp::new(QByteArray).optional(),
        body: ItemProp::new(QString).optional(),
        time: ItemProp::new(Qint64).optional(),
        has_attachments: ItemProp::new(Bool).optional()
    };

    let funcs = functions! {
        mut clearSearch() => Void,
    };

    obj! {
        GlobalMessageSearch: Obj::new().list().funcs(funcs).props(props).item_props(item_props)
    }
}
