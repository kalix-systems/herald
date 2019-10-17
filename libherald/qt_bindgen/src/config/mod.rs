// #![allow(unused)]
use rust_qt_binding_generator::configuration::*;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::rc::Rc;

mod func;
mod item_prop;
mod macros;
mod obj;
mod prop;

use func::*;
use item_prop::*;
use obj::*;
use prop::*;

use crate::*;

pub(crate) fn get() -> Config {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let parent_dir = crate_dir.parent().unwrap();

    // ignore this
    let mut config_file = parent_dir.clone().to_path_buf();
    config_file.push("bindings.json");

    let cpp_file = PathBuf::from("qt_ffi/Bindings.cpp");

    let objects = objects();
    let rust = Rust {
        dir: parent_dir.to_path_buf(),
        implementation_module: "implementation".into(),
        interface_module: "interface".into(),
    };
    let rust_edition = RustEdition::Rust2018;
    let overwrite_implementation = false;

    Config {
        // ignore this
        config_file,
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
       network_handle(),
       users(),
       members(),
       messages(),
       config_obj(),
       conversation_builder(),
       message_builder(),
       attachments()
    }
}

fn herald_state() -> Object {
    let properties = props! {
        configInit: Prop::new().simple(Bool).write()
    };

    obj! {
        HeraldState : Obj::new().props(properties)
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
    ItemProp::new(SimpleType::Bool).write()
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
       matched: matched_item_prop().write(),
       picture: picture_item_prop().write(),
       color: color_item_prop().write()
    };

    let funcs = functions! {
        mut removeConversation(row_index: QUint64) => Bool,
        mut pollUpdate() => Bool,
        mut toggleFilterRegex() => Bool,
    };

    obj! {
       Conversations: Obj::new().list().props(props).item_props(item_props).funcs(funcs)
    }
}

fn network_handle() -> Object {
    let props = props! {
        connectionUp: Prop::new().simple(Bool),
        connectionPending: Prop::new().simple(Bool),
        membersData: Prop::new().simple(QUint8)
    };

    let funcs = functions! {
        mut registerNewUser(user_id: QString) => Bool,
        mut login() => Bool,
        const sendAddRequest(user_id: QString, conversation_id: QByteArray) => Bool,
    };

    obj! {
        NetworkHandle: Obj::new().props(props).funcs(funcs)
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
        mut pollUpdate() => Bool,
        mut toggleFilterRegex() => Bool,
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
        mut pollUpdate() => Bool,
        mut  toggleFilterRegex() => Bool,
    };

    obj! {
        Members: Obj::new().list().props(props).funcs(funcs).item_props(item_props)
    }
}

fn messages() -> Object {
    let props = props! {
        conversationId: conv_id_prop(),
        lastAuthor: Prop::new().simple(QString).optional(),
        lastBody: Prop::new().simple(QString).optional(),
        lastEpochTimestampMs: Prop::new().simple(Qint64).optional(),
        lastStatus: Prop::new().simple(QUint32).optional()
    };

    let item_props = item_props! {
        messageId: ItemProp::new(QByteArray),
        author: ItemProp::new(QString),
        body: ItemProp::new(QString).optional(),
        epochTimestampMs: ItemProp::new(Qint64),
        op: ItemProp::new(QByteArray),
        receiptStatus: ItemProp::new(QUint32)
    };

    let funcs = functions! {
        //mut sendMessage(body: QString) => Bool,
        //mut reply(body: QString, op: QByteArray) => Bool,
        mut deleteMessage(row_index: QUint64) => Bool,
        mut clearConversationHistory() => Bool,
        mut pollUpdate() => Bool,
        const messageBodyById(msg_id: QByteArray) => QString,
        const messageAuthorById(msg_id: QByteArray) => QString,
        const indexById(msg_id: QByteArray) => Qint64,
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
        colorscheme: Prop::new().simple(QUint32).write()
    };

    obj! {
        Config: Obj::new().props(props)
    }
}

fn conversation_builder() -> Object {
    let item_prop = item_props! {
        memberId: ItemProp::new(QString)
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
        ConversationBuilder: Obj::new().list().funcs(funcs).item_props(item_prop)
    }
}

fn message_builder() -> Object {
    let props = props! {
        // Conversation id
        conversationId: conv_id_prop(),
        // Message id of the message being replied to
        replyingTo: Prop::new().simple(QByteArray).optional().write(),
        // Body of the messagee
        body: Prop::new().simple(QString).optional().write()
    };

    let item_props = item_props! {
        attachmentPath: ItemProp::new(QString)
    };

    let funcs = functions! {
        mut finalize() => Void,
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
