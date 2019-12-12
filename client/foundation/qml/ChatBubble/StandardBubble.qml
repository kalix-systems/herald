import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

ColumnLayout {
    id: bubbleRoot
    property real maxWidth: imageAttach ? 300 : Math.min(parent.maxWidth, 600)
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string authorName: ""
    property color authorColor
    spacing: 0
    property bool expanded: false
    property bool elided: false
    property bool imageAttach: false
    property bool docAttach: false
    property bool reply: false
    property string medAttachments
    property string documentAttachments
    property var replyId
    property var messageModelData

    // Text edit alias
    property alias messageBody: messageBody

    // all messages are un-expanded on completion
    Component.onCompleted: bubbleRoot.expanded = false

    //image component
    Component {
        id: image
        AttachmentContent {}
    }

    // document component
    Component {
        id: doc
        FileAttachmentContent {}
    }

    // reply bubble if there is doc file content
    Component {
        id: replyDocContent
        ReplyDocBubble {
            maxWidth: bubbleRoot.maxWidth
            replyId: bubbleRoot.replyId
            modelData: bubbleRoot.messageModelData
        }
    }

    // reply bubble if there is no doc file content
    Component {
        id: replyContent
        ReplyBubble {
            maxWidth: bubbleRoot.maxWidth
            replyId: bubbleRoot.replyId
            modelData: bubbleRoot.messageModelData
        }
    }

    //reply bubble loader
    Loader {
        sourceComponent: {
            if (reply) {
                if (messageModelData.opDocAttachments.length === 0)
                    return replyContent
                return replyDocContent
            }
            return undefined
        }
    }

    //author name
    ChatLabel {
        id: uname
        senderName: authorName
        senderColor: authorColor
    }

    //media and file column loader
    Column {
        Loader {
            id: imageLoader
            sourceComponent: imageAttach ? image : undefined
        }

        Loader {
            id: fileLoader
            sourceComponent: docAttach ? doc : undefined
        }
    }

    //message body
    StandardTextEdit {
        id: messageBody
        Layout.maximumWidth: bubbleRoot.imageAttach ? 300 : Math.min(
                                                          bubbleRoot.maxWidth,
                                                          600)
    }
    ElideHandler {}

    StandardStamps {}
}
