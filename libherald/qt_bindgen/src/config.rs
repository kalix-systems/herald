use builders::func::*;
use builders::item_prop::*;
use builders::obj::*;
use builders::prop::*;
use builders::sig::*;
use riqtshaw::{
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

    let overwrite_implementation = false;

    Config {
        out_dir: parent_dir.to_path_buf(),
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

       conversation_content(),
       messages(),
       message_builder(),

       media_attachments(),
       document_attachments(),
       emoji_picker()
    }
}

fn herald() -> Object {
    let properties = props! {
        configInit: Prop::new().simple(Bool),
        connectionUp: Prop::new().simple(Bool),
        connectionPending: Prop::new().simple(Bool),

        // Registration failure code
        // UserIdTaken  => 0,
        // KeyTaken     => 1,
        // BadSignature => 2,
        // Other        => 3,
        registrationFailureCode: Prop::new().simple(QUint8).optional(),

        // object props
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
        mut registerNewUser(user_id: QString, addr: QString, port: QString) => Void,
        mut login() => Bool,
        mut setAppLocalDataDir(path: QString) => Void,
        mut pollUpdate() => Void,
    };

    let hooks = signals! {
        tryPoll(),
        | connect tryPoll pollUpdate
    };

    obj! {
        Herald: Obj::new().props(properties).funcs(funcs).hooks(hooks)
    }
}

fn errors() -> Object {
    let functions = functions! {
        mut nextError() => QString,
    };

    let hooks = signals! {
       newError(), |
    };

    obj! {
        Errors: Obj::new().funcs(functions).hooks(hooks)
    }
}

fn utils() -> Object {
    let functions = functions! {
        const compareByteArray(bs1: QByteArray, bs2: QByteArray) => Bool,
        const isValidRandId(bs: QByteArray) => Bool,
        const saveFile(fpath: QString, target_path: QString) => Bool,
        // Returns image dimensions of the image at `path`, serialized as JSON
        const imageDimensions(path: QString) => QString,
        // Strips QML URL prefix
        const stripUrlPrefix(path: QString) => QString,
        // Given image dimensions and a constant, scales the smaller dimension down
        // and makes the larger dimension equal to the constant
        const imageScaling(path: QString, scale: QUint32) => QString,
        const imageScaleReverse(path: QString, scale: QUint32) => QString,
    };

    obj! {
        Utils: Obj::new().funcs(functions)
    }
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
       picture: picture_item_prop().get_by_value(),
       color: color_item_prop(),
       status: ItemProp::new(QUint8).write()
    };

    let funcs = functions! {
        mut removeConversation(row_index: QUint64) => Bool,
        mut toggleFilterRegex() => Bool,
        mut clearFilter() => Void,
        // `profile_picture` is a path and bounding rectangle encoded as JSON.
        // See `heraldcore/image_utils`.
        mut setProfilePicture(index: QUint64, profile_picture: QString) => Void,
        const indexById(conversation_id: QByteArray) => Qint64,
    };

    obj! {
       Conversations: Obj::new().list().props(props).item_props(item_props).funcs(funcs)
    }
}

fn users() -> Object {
    let props = filter_props();

    let item_props = item_props! {
       userId: ItemProp::new(QString),
       name: ItemProp::new(QString).get_by_value(),
       pairwiseConversationId: ItemProp::new(QByteArray).get_by_value(),
       status: ItemProp::new(QUint8).write(),
       matched: matched_item_prop(),
       profilePicture: picture_item_prop().get_by_value(),
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

fn conversation_content() -> Object {
    let props = props! {
        members: Prop::new().object(members()),
        messages: Prop::new().object(messages()),
        conversationId: conv_id_prop()
    };

    let funcs = functions! {
        mut pollUpdate() => Void,
    };

    let hooks = signals! {
        tryPoll(),
        | connect tryPoll pollUpdate
    };

    obj! {
        ConversationContent: Obj::new().props(props).funcs(funcs).hooks(hooks)
    }
}

fn members() -> Object {
    let mut props = props! {};
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

fn emoji_picker() -> Object {
    let props = props! {
        smileys_index: Prop::new().simple(QUint32),
        nature_index: Prop::new().simple(QUint32),
        body_index: Prop::new().simple(QUint32),
        food_index: Prop::new().simple(QUint32),
        locations_index: Prop::new().simple(QUint32),
        activities_index: Prop::new().simple(QUint32),
        symbols_index: Prop::new().simple(QUint32),
        flags_index: Prop::new().simple(QUint32),
        objects_index: Prop::new().simple(QUint32)
    };

    let funcs = functions! {
        mut clearSearch() => Void,
        mut setSearchString(search_string: QString) => Void,
    };

    let item_props = item_props! {
     emoji: ItemProp::new(QString),
     skintone_modifier: ItemProp::new(Bool)
    };

    obj! {
        EmojiPicker: Obj::new().list().funcs(funcs).props(props).item_props(item_props)
    }
}

fn messages() -> Object {
    let props = props! {
        lastAuthor: Prop::new().simple(QString).optional().get_by_value(),
        lastBody: Prop::new().simple(QString).optional().get_by_value(),
        // Insertion time of last available message
        lastTime: Prop::new().simple(Qint64).optional(),
        lastStatus: Prop::new().simple(QUint32).optional(),
        lastAuxCode: Prop::new().simple(QUint8).optional(),
        lastHasAttachments: Prop::new().simple(Bool).optional(),

        // User id of the last user to send a typing notification
        typingUserId: Prop::new().simple(QString).optional(),

        isEmpty: Prop::new().simple(Bool),

        searchPattern: filter_prop(),
        searchRegex: filter_regex_prop(),
        searchActive: Prop::new().simple(Bool).write(),
        // Number of search results
        searchNumMatches: Prop::new().simple(QUint64),
        // Position in search results of focused item, e.g., 4 out of 7
        searchIndex: Prop::new().simple(QUint64),

        builder: Prop::new().object(message_builder())
    };

    let item_props = item_props! {
        // Main message properties
        msgId: ItemProp::new(QByteArray).optional(),
        // Author of the message
        author: ItemProp::new(QString).optional().get_by_value(),
        // Message body. Possibly truncated if the message is too long
        body: ItemProp::new(QString).get_by_value(),
        // Full message body
        fullBody: ItemProp::new(QString).get_by_value(),
        // Time the message was saved locally
        insertionTime: ItemProp::new(Qint64).optional(),
        // Time the message arrived at the server (only valid for inbound messages)
        serverTime: ItemProp::new(Qint64).optional(),
        // Time the message will expire, if ever
        expirationTime: ItemProp::new(Qint64).optional(),

        // User profile picture
        authorProfilePicture: ItemProp::new(QString).get_by_value(),
        // User color
        authorColor: ItemProp::new(QUint32).optional(),
        // User name
        authorName: ItemProp::new(QString).optional().get_by_value(),

        // Message reactions
        reactions: ItemProp::new(QString).get_by_value(),
        // Auxiliary message data, serialized as JSON
        auxData: ItemProp::new(QString).get_by_value(),

        // Media attachments metadata, serialized as JSON
        mediaAttachments: ItemProp::new(QString).get_by_value(),
        // Full media attachments metadata, serialized as JSON
        fullMediaAttachments: ItemProp::new(QString).get_by_value(),
        // Document attachments metadata, serialized as JSON
        docAttachments: ItemProp::new(QString).get_by_value(),

        userReceipts: ItemProp::new(QString).get_by_value(),
        receiptStatus: ItemProp::new(QUint32).optional(),

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
        opMsgId: ItemProp::new(QByteArray).optional().get_by_value(),
        opAuthor: ItemProp::new(QString).optional().get_by_value(),
        opName: ItemProp::new(QString).optional().get_by_value(),
        opBody: ItemProp::new(QString).get_by_value(),
        opInsertionTime: ItemProp::new(Qint64).optional(),
        opExpirationTime: ItemProp::new(Qint64).optional(),
        opColor: ItemProp::new(QUint32).optional(),
        // Auxiliary message data, serialized as JSON
        opAuxData: ItemProp::new(QString).get_by_value(),

        // Media attachments metadata, serialized as JSON
        opMediaAttachments: ItemProp::new(QString).get_by_value(),
        // Document attachments metadata, serialized as JSON
        opDocAttachments: ItemProp::new(QString).get_by_value()
    };

    let hooks = signals! {
       newTypingIndicator(),|
    };

    let funcs = functions! {
        mut deleteMessage(row_index: QUint64) => Bool,
        mut deleteMessageById(id: QByteArray) => Bool,
        mut markReadById(id: QByteArray) => Void,
        mut clearConversationHistory() => Bool,
        mut clearSearch() => Void,
        mut nextSearchMatch() => Qint64,
        mut prevSearchMatch() => Qint64,
        mut setSearchHint(scrollbar_position: Float, scrollbar_height: Float) => Void,
        mut setElisionLineCount(line_count: QUint8) => Void,
        mut setElisionCharCount(char_count: QUint16) => Void,
        mut setElisionCharsPerLine(chars_per_line: QUint8) => Void,
        mut addReaction(index: QUint64, content: QString) => Void,
        mut removeReaction(index: QUint64, content: QString) => Void,
        mut sendTypingIndicator() => Void,
        const indexById(msg_id: QByteArray) => Qint64,
        const saveAllAttachments(index: QUint64, dest: QString) => Bool,
    };

    obj! {
        Messages: Obj::new().list().funcs(funcs).item_props(item_props).props(props).hooks(hooks)
    }
}

fn message_builder() -> Object {
    let props = props! (
        isReply: Prop::new().simple(Bool),
        // Body of the message
        body: Prop::new().simple(QString).optional().write(),
        // Set expiration period of the message.
        expirationPeriod: Prop::new().simple(QUint8).write().optional(),


        hasMediaAttachment: Prop::new().simple(Bool),
        hasDocAttachment: Prop::new().simple(Bool),
        documentAttachments: Prop::new().object(document_attachments()),
        mediaAttachments: Prop::new().object(media_attachments()),

        // Message id of the message being replied to, if any
        opId: Prop::new().simple(QByteArray).optional().write(),
        opAuthor: Prop::new().simple(QString).optional(),
        opBody: Prop::new().simple(QString).optional(),
        opTime: Prop::new().simple(Qint64).optional(),
        // Media attachments metadata, serialized as JSON
        opMediaAttachments: Prop::new().simple(QString),
        // Document attachments metadata, serialized as JSON
        opDocAttachments: Prop::new().simple(QString),
        // Aux content metadata, serialized as JSON
        opAuxContent: Prop::new().simple(QString),
        // Time the message will expire, if ever
        opExpirationTime: Prop::new().simple(Qint64).optional()
    );

    let funcs = functions! {
        mut finalize() => Void,
        mut clearReply() => Void,
        mut addAttachment(path: QString) => Bool,
        mut removeDoc(row_index: QUint64) => Bool,
        mut removeMedia(row_index: QUint64) => Bool,
    };

    obj! {
        MessageBuilder: Obj::new().list().funcs(funcs).props(props)
    }
}

fn config() -> Object {
    let props = props! {
        configId: Prop::new().simple(QString),
        name: Prop::new().simple(QString).write(),
        profilePicture: Prop::new().simple(QString).optional(),
        color: Prop::new().simple(QUint32).write(),
        ntsConversationId: Prop::new().simple(QByteArray),
        preferredExpiration: Prop::new().simple(QUint8).write()
    };

    let funcs = functions! {
        // `profile_picture` is a path and bounding rectangle encoded as JSON.
        // See `heraldcore/image_utils`.
        mut setProfilePicture(profile_picture: QString) => Void,
    };

    obj! {
        Config: Obj::new().props(props).funcs(funcs)
    }
}

fn conversation_builder() -> Object {
    let item_prop = item_props! {
        memberId: ItemProp::new(QString),
        memberColor: ItemProp::new(QUint32),
        memberName: ItemProp::new(QString).get_by_value(),
        memberProfilePicture: ItemProp::new(QString).get_by_value()
    };

    let prop = props! {
        picture: Prop::new().simple(QString).optional()
    };

    let funcs = functions! {
        mut addMember(user_id: QString) => Bool,
        mut removeMemberById(user_id: QString) => Bool,
        mut removeMemberByIndex(index: QUint64) => Bool,
        mut removeLast() => Void,
        mut finalize() => Void,
        mut clear() => Void,

        mut setTitle(title: QString) => Void,
        // `profile_picture` is a path and bounding rectangle encoded as JSON.
        // See `heraldcore/image_utils`.
        mut setProfilePicture(profile_picture: QString) => Void,
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

fn media_attachments() -> Object {
    let item_props = item_props! {
        // Path the the attachment
        mediaAttachmentPath: ItemProp::new(QString)
    };

    obj! {
        MediaAttachments: Obj::new().list().item_props(item_props)
    }
}

fn document_attachments() -> Object {
    let item_props = item_props! {
        // File name
        documentAttachmentName: ItemProp::new(QString).get_by_value(),
        documentAttachmentSize: ItemProp::new(QUint64)
    };

    obj! {
        DocumentAttachments: Obj::new().list().item_props(item_props)
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
        // Conversation id
        conversation: ItemProp::new(QByteArray).optional(),
        // Is the conversation pairwise?
        conversationPairwise: ItemProp::new(Bool).optional(),
        // Path to conversation picture, if it exists
        conversationPicture: ItemProp::new(QString).optional().get_by_value(),
        conversationColor: ItemProp::new(QUint32).optional().get_by_value(),
        conversationTitle: ItemProp::new(QString).optional().get_by_value(),
        beforeFirstMatch: ItemProp::new(QString),
        firstMatch: ItemProp::new(QString),
        afterFirstMatch: ItemProp::new(QString),
        time: ItemProp::new(Qint64).optional()
    };

    let funcs = functions! {
        mut clearSearch() => Void,
    };

    obj! {
        MessageSearch: Obj::new().list().funcs(funcs).props(props).item_props(item_props)
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
