import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../js/utils.mjs" as Utils

ColumnLayout {

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property color opColor: CmnCfg.avatarColors[contactsModel.colorById(
                                                    replyPreview.author)]
    property string authorName: ""
    spacing: 0
    property color authorColor
    property var replyId


    MessagePreview {
        id: replyPreview
        messageId: replyId === undefined ? null : replyId
    }

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: reply.implicitHeight
        color: CmnCfg.palette.sideBarHighlightColor
        Layout.margins: CmnCfg.margin / 2
       // Layout.topMargin: 0
        Layout.minimumWidth: reply.width

        Rectangle {
            visible: !replyPreview.isDangling
            id: verticalAccent
            anchors.right: !outbound ? replyWrapper.left : undefined
            anchors.left: outbound ? replyWrapper.right : undefined
            height: replyWrapper.height
            width: CmnCfg.smallMargin / 4
            color: opColor
        }

        ColumnLayout {
            id: reply
            spacing: 0

            Label {
                id: opLabel
                text: !replyPreview.isDangling ? contactsModel.nameById(
                                                     replyPreview.author) : ""
                font.bold: true
                Layout.topMargin: CmnCfg.margin / 2
                Layout.bottomMargin: 0
                Layout.leftMargin: CmnCfg.smallMargin
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

            Label {
                Layout.leftMargin: 8
                Layout.bottomMargin: 5
                Layout.topMargin: 0
                   font.pixelSize: 10
                   text: !replyPreview.isDangling ?
                             Utils.friendlyTimestamp(
                             replyPreview.epochTimestampMs) : ""
                   color: CmnCfg.palette.secondaryTextColor
               }
        }
    }

    ChatLabel {
        id: uname
        senderName: authorName
        senderColor: authorColor
    }

    StandardTextEdit {
        id: messageBody
    }

    StandardStamps {}
}
