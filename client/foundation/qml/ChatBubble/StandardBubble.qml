import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

ColumnLayout {
    id: wrapperCol
    property real maxWidth: Math.min(parent.maxWidth, 600)
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string authorName: ""
    property color authorColor
    spacing: 0
    property bool expanded: false
    property bool elided: false
    property bool attach: false
    property bool reply: false
    property string medAttachments
    property string documentAttachments
    property var replyId

    ChatLabel {
        id: uname
        senderName: authorName
        senderColor: authorColor
    }

    Component {
        id: image
        AttachmentContent {
            maxWidth: wrapperCol.maxWidth
            medAttachments: wrapperCol.medAttachments
            documentAttachments: wrapperCol.documentAttachments
        }
    }

    Component {
        id: replyContent
        ReplyBubble {
            maxWidth: wrapperCol.maxWidth
            replyId: wrapperCol.replyId
        }
    }

    Column {
        Loader {
            sourceComponent: reply ? replyContent : undefined
        }

        Loader {
            id: imageLoader
            sourceComponent: attach ? image : undefined
        }
    }

    Component.onCompleted: wrapperCol.expanded = false

    StandardTextEdit {
        id: messageBody
    }
    ElideHandler {}

    StandardStamps {}
}
