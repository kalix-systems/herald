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
       conversation_builder()
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
        nextError: Func::new(QString).mutable()
    };

    obj! {
        Errors: Obj::new().props(properties).funcs(functions)
    }
}

fn herald_utils() -> Object {
    let functions = functions! {
        compareByteArray: Func::new(Bool).arg("bs1", QByteArray).arg("bs2", QByteArray),
        isValidRandId: Func::new(Bool).arg("bs", QByteArray)
    };

    obj! {
        HeraldUtils: Obj::new().funcs(functions)
    }
}

fn toggle_filter_regex_func() -> Func<'static> {
    Func::new(SimpleType::Bool).mutable()
}

fn poll_update_func() -> Func<'static> {
    Func::new(SimpleType::Bool).mutable()
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
    ItemProp::new(SimpleType::QUint32).write()
}

fn picture_item_prop() -> ItemProp {
    ItemProp::new(SimpleType::QString).write().optional()
}

fn conversations() -> Object {
    let props = filter_props();

    let item_props = item_props! {
       conversationId: ItemProp::new(QByteArray),
       title: ItemProp::new(QString).write(),
       muted: ItemProp::new(Bool).write(),
       pairwise: ItemProp::new(Bool),
       matched: matched_item_prop(),
       picture: picture_item_prop(),
       color: color_item_prop()
    };

    let funcs = functions! {
        removeConversation: Func::new(Bool).mutable().arg("row_index", QUint64),
        toggleFilterRegex: toggle_filter_regex_func(),
        pollUpdate: Func::new(Bool).mutable()
    };

    obj! {
       Conversations: Obj::new().list().props(props).item_props(item_props).funcs(funcs)
    }
}

fn network_handle() -> Object {
    let props = props! {
        connectionUp: Prop::new().simple(Bool),
        connectionPending: Prop::new().simple(Bool),
        msgData: Prop::new().simple(QUint8),
        membersData: Prop::new().simple(QUint8)
    };

    let funcs = functions! {
        sendAddRequest: Func::new(Bool).arg("user_id", QString).arg("conversation_id", QByteArray),
        registerNewUser: Func::new(Bool).arg("user_id", QString),
        login: Func::new(Bool)
    };

    obj! {
        Conversations: Obj::new().props(props).funcs(funcs)
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
       profilePicture: picture_item_prop(),
       color: color_item_prop()
    };

    let funcs = functions! {
        add: Func::new(Bool).mutable().arg("id", QString),
        colorById: Func::new(QUint32).arg("id", QString),
        nameById: Func::new(QString).arg("id", QString),
        profilePictureById: Func::new(QString).arg("id", QString),
        toggleFilterRegex: toggle_filter_regex_func(),
        pollUpdate: poll_update_func()
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
       name: ItemProp::new(QString).get_by_value().write(),
       pairwiseConversationId: ItemProp::new(QByteArray).get_by_value(),
       status: ItemProp::new(QUint8).write(),
       matched: matched_item_prop(),
       profilePicture: picture_item_prop(),
       color: color_item_prop()
    };

    let funcs = functions! {
        addToConversation: Func::new(Bool).mutable().arg("id", QString),
        removeFromConversationByIndex: Func::new(Bool).mutable().arg("row_index", QUint64),
        toggleFilterRegex: toggle_filter_regex_func(),
        pollUpdate: poll_update_func()
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
        lastEpochTimestampMs: Prop::new().simple(QUint64).optional(),
        lastStatus: Prop::new().simple(QUint32).optional()
    };

    let item_props = item_props! {
        messageId: ItemProp::new(QByteArray),
        author: ItemProp::new(QString),
        body: ItemProp::new(QString),
        epochTimestampMs: ItemProp::new(QUint64),
        op: ItemProp::new(QByteArray),
        receiptStatus: ItemProp::new(QUint32)
    };

    let funcs = functions! {
        sendMessage: Func::new(Bool).mutable().arg("body", QString),
        reply: Func::new(Bool).mutable().arg("body", QString).arg("op", QByteArray),
        messageBodyById: Func::new(QString).arg("msg_id", QByteArray),
        messageAuthorById: Func::new(QString).arg("msg_id", QByteArray),
        indexById: Func::new(QUint64).arg("msg_id", QByteArray),
        deleteMessage: Func::new(Bool).arg("row_index", QUint64),
        clearConversationHistory: Func::new(Bool).mutable(),
        pollUpdate: poll_update_func()
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
    let props = props! {
        title: Prop::new().simple(QString).optional()
    };

    let item_prop = item_props! {
        memberId: ItemProp::new(QString)
    };

    let funcs = functions! {
        addMember: Func::new(Bool).mutable().arg("user_id", QString),
        removeMemberById: Func::new(Bool).mutable().arg("user_id", QString),
        removeMemberByIndex: Func::new(Bool).mutable().arg("index", QUint64),
        removeLast: Func::new(Void).mutable(),
        finalize: Func::new(Bool).mutable()
    };

    obj! {
        ConversationBuilder: Obj::new().list().funcs(funcs).props(props).item_props(item_prop)
    }
}
