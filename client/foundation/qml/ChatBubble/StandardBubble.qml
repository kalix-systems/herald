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

    ChatLabel {
        id: uname
        senderName: authorName
        senderColor: authorColor
    }

    Component {
        id: image
        AttachmentContent {}
    }

    Component {
        id: replyContent
        ReplyBubble {
            maxWidth: bubbleRoot.maxWidth
            replyId: bubbleRoot.replyId
        }
    }

    Component {
        id: doc
        FileAttachmentContent {}
    }

    Column {
        // width: background.width
        Loader {
            sourceComponent: reply ? replyContent : undefined
        }

        Loader {
            id: imageLoader
            sourceComponent: imageAttach ? image : undefined
        }

        Loader {
            id: fileLoader
            sourceComponent: docAttach ? doc : undefined
        }
    }

    Component.onCompleted: {
        bubbleRoot.expanded = false
    }

    StandardTextEdit {
        id: messageBody
        Layout.maximumWidth: bubbleRoot.imageAttach ? 300 : Math.min(
                                                          bubbleRoot.maxWidth,
                                                          600)
    }
    ElideHandler {}

    StandardStamps {}
}
