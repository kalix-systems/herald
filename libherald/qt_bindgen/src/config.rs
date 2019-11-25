use builders::func::*;
use builders::item_prop::*;
use builders::obj::*;
use builders::prop::*;
use rust_qt_binding_generator::{
    builders,
    configuration::{SimpleType::*, *},
};
use std::{collections::BTreeMap, path::PathBuf, rc::Rc};

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
       herald(),
       users(),
       config(),
       conversations(),
       message_search(),
       conversation_builder(),
       users_search(),
       utils(),
       errors(),

       members(),

       messages(),
       message_builder(),
       attachments()
    }
}

fn herald() -> Object {
    let properties = props! {
        configInit: Prop::new().simple(Bool),
        connectionUp: Prop::new().simple(Bool),
        connectionPending: Prop::new().simple(Bool),

        config: Prop::new().object(config()),
        conversationBuilder: Prop::new().object(conversation_builder()),
        conversations: Prop::new().object(conversations()),
        errors: Prop::new().object(errors()),
        messageSearch: Prop::new().object(message_search()),
        users: Prop::new().object(users()),
        usersSearch: Prop::new().object(users_search()),
        utils: Prop::new().object(utils())
    };

    let funcs = functions! {
        mut registerNewUser(user_id: QString) => Void,
        mut login() => Bool,
    };

    obj! {
        Herald: Obj::new().props(properties).funcs(funcs).list()
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

fn utils() -> Object {
    let functions = functions! {
        const compareByteArray(bs1: QByteArray, bs2: QByteArray) => Bool,
        const isValidRandId(bs: QByteArray) => Bool,
    };

    obj! {
        Utils: Obj::new().funcs(functions)
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
       conversationId: ItemProp::new(QByteArray).get_by_value(),
       title: ItemProp::new(QString).write().optional().get_by_value(),
       muted: ItemProp::new(Bool).write(),
       pairwise: ItemProp::new(Bool),
       expirationPeriod: ItemProp::new(QUint8).write(),
       matched: matched_item_prop(),
       picture: picture_item_prop().write().get_by_value(),
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

fn messages() -> Object {
    let props = props! {
        conversationId: conv_id_prop(),
        lastAuthor: Prop::new().simple(QString).optional(),
        lastBody: Prop::new().simple(QString).optional(),
        // Insertion time of last available message
        lastTime: Prop::new().simple(Qint64).optional(),
        lastStatus: Prop::new().simple(QUint32).optional(),
        isEmpty: Prop::new().simple(Bool),
        searchPattern: filter_prop(),
        searchRegex: filter_regex_prop(),
        searchActive: Prop::new().simple(Bool).write(),
        // Number of search results
        searchNumMatches: Prop::new().simple(QUint64),
        // Position in search results of focused item, e.g., 4 out of 7
        searchIndex: Prop::new().simple(QUint64),

        builder: Prop::new().object(message_builder()),
        // Id of the message the message builder is replying to, if any
        builderOpMsgId: Prop::new().simple(QByteArray).optional().write()
    };

    let item_props = item_props! {
        // Main message properties
        msgId: ItemProp::new(QByteArray).optional(),
        author: ItemProp::new(QString).optional(),
        body: ItemProp::new(QString).optional(),
        insertionTime: ItemProp::new(Qint64).optional(),
        serverTime: ItemProp::new(Qint64).optional(),
        expirationTime: ItemProp::new(Qint64).optional(),
        hasAttachments: ItemProp::new(Bool).optional(),
        receiptStatus: ItemProp::new(QUint32).optional(),
        dataSaved: ItemProp::new(Bool).optional(),
        isHead: ItemProp::new(Bool).optional(),
        isTail: ItemProp::new(Bool).optional(),

        // 0 => Not matched,
        // 1 => Matched,
        // 2 => Matched and focused
        matchStatus: ItemProp::new(QUint8).optional(),

        // 0 => Not reply
        // 1 => Dangling (i.e., unknown reply)
        // 2 => Known reply
        replyType: ItemProp::new(QUint8).optional(),

        // Message preview properties
        opMsgId: ItemProp::new(QByteArray).optional(),
        opAuthor: ItemProp::new(QString).optional(),
        opBody: ItemProp::new(QString).optional(),
        opInsertionTime: ItemProp::new(Qint64).optional(),
        opExpirationTime: ItemProp::new(Qint64).optional(),
        opHasAttachments: ItemProp::new(Bool).optional()
    };

    let funcs = functions! {
        mut deleteMessage(row_index: QUint64) => Bool,
        mut clearConversationHistory() => Bool,
        mut clearSearch() => Void,
        mut nextSearchMatch() => Qint64,
        mut prevSearchMatch() => Qint64,
        mut setSearchHint(scrollbar_position: Float, scrollbar_height: Float) => Void,
        const indexById(msg_id: QByteArray) => QUint64,
    };

    obj! {
        Messages: Obj::new().list().funcs(funcs).item_props(item_props).props(props)
    }
}

fn message_builder() -> Object {
    let props = props! (
        isReply: Prop::new().simple(Bool),
        // Body of the message
        body: Prop::new().simple(QString).optional().write(),
        isMediaMessage: Prop::new().simple(Bool),
        parseMarkdown: Prop::new().simple(Bool).write(),

        // Message id of the message being replied to, if any
        opId: Prop::new().simple(QByteArray).optional(),
        opAuthor: Prop::new().simple(QString).optional(),
        opBody: Prop::new().simple(QString).optional(),
        opTime: Prop::new().simple(Qint64).optional(),
        opHasAttachments: Prop::new().simple(Bool).optional()
    );

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

fn config() -> Object {
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
        mut clear() => Void,
    };

    obj! {
        ConversationBuilder: Obj::new().list().funcs(funcs).item_props(item_prop).props(prop)
    }
}

fn users_search() -> Object {
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
        mut refresh() => Void,
    };

    obj! {
        UsersSearch: Obj::new().list().props(props).funcs(funcs).item_props(item_props)
    }
}

fn attachments() -> Object {
    let props = props! {
        // the message id the attachments list is associated with
        attachmentsMsgId: Prop::new().simple(QByteArray).optional().write()
    };

    let item_props = item_props! {
        // Path the the attachment
        attachmentPath: ItemProp::new(QString)
    };

    obj! {
        Attachments: Obj::new().list().props(props).item_props(item_props)
    }
}

fn message_search() -> Object {
    let props = props! {
        searchPattern: Prop::new().simple(QString).optional().write(),
        regexSearch: Prop::new().simple(Bool).optional().write()
    };

    let item_props = item_props! {
        msgId: ItemProp::new(QByteArray).optional(),
        author: ItemProp::new(QString).optional(),
        conversation: ItemProp::new(QByteArray).optional(),
        conversationPairwise: ItemProp::new(Bool).optional(),
        conversationPicture: ItemProp::new(QString).optional().get_by_value(),
        conversationColor: ItemProp::new(QUint32).optional().get_by_value(),
        conversationTitle: ItemProp::new(QString).optional().get_by_value(),
        body: ItemProp::new(QString).optional(),
        time: ItemProp::new(Qint64).optional(),
        has_attachments: ItemProp::new(Bool).optional()
    };

    let funcs = functions! {
        mut clearSearch() => Void,
    };

    obj! {
        MessageSearch: Obj::new().list().funcs(funcs).props(props).item_props(item_props)
    }
}
