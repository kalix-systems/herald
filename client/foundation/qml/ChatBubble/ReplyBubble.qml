import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

ColumnLayout {

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property color opColor: CmnCfg.avatarColors[contactsModel.colorById(
                                                    replyPreview.author)]
    property string authorName: ""
    property int spacing: 0
    property color authorColor
    property var replyId

    ChatLabel {
        id: sender
        senderName: authorName
        senderColor: authorColor
    }

    MessagePreview {
        id: replyPreview
        messageId: replyId === undefined ? null : replyId
    }

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: reply.implicitHeight
        color: Qt.lighter(CmnCfg.palette.tertiaryColor, 1.3)
        Layout.margins: CmnCfg.smallMargin
        Layout.topMargin: 0
        Layout.minimumWidth: reply.width

        Rectangle {
            visible: !replyPreview.isDangling
            id: verticalAccent
            anchors.right: !outbound ? replyWrapper.left : undefined
            anchors.left: outbound ? replyWrapper.right : undefined
            height: replyWrapper.height
            width: CmnCfg.smallMargin / 2
            color: opColor
        }

        ColumnLayout {
            id: reply
            spacing: 1

            Label {
                id: opLabel
                text: !replyPreview.isDangling ? contactsModel.nameById(replyPreview.author) : ""
                font.bold: true
                Layout.margins: CmnCfg.smallMargin
                Layout.bottomMargin: 0
                Layout.leftMargin: CmnCfg.smallMargin / 2
                Layout.preferredHeight: !replyPreview.isDangling ? implicitHeight : 0
                color: opColor
            }

            TextMetrics {
                readonly property real constWidth: replyBody.width * 3
                id: opBodyTextMetrics
                text: !replyPreview.isDangling ? replyPreview.body : "Original message not found"
                elideWidth: constWidth
                elide: Text.ElideRight
            }

            StandardTextEdit {
                id: replyBody
                text: opBodyTextMetrics.elidedText
                Layout.minimumWidth: messageBody.width
            }
        }
    }

    StandardTextEdit {
        id: messageBody
    }

    StandardStamps {
    }
}
