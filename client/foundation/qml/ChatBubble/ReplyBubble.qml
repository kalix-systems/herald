import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../js/utils.mjs" as Utils
import QtQuick 2.13

ColumnLayout {
    id: wrapperCol

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
    property alias jumpHandler: jumpHandler
    property alias replyHighlightAnimation: replyHighlightAnimation


    MessagePreview {
        id: replyPreview
        messageId: replyId === undefined ? null : replyId
    }

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: reply.implicitHeight
        color: CmnCfg.palette.sideBarHighlightColor
        Layout.margins: CmnCfg.margin / 2
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
        MouseArea {
            anchors.centerIn: reply
            width: reply.width
            height: reply.height
            z: 10
            id: jumpHandler
        }

        //TODO: nicer animation
        SequentialAnimation {
            id: replyHighlightAnimation
        PropertyAnimation {
            target: chatListView.itemAt(ownedConversation.indexById(replyId))
            property: "highlight.opacity"
            to: 5
            duration: 200
            easing.type: Easing.InOutQuad
         }

        PropertyAnimation {
            target: chatListView.itemAt(ownedConversation.indexById(replyId))
            property: "highlight.opacity"
            to: -5
            duration: 200
            easing.type: Easing.InOutQuad
        }
        }

        ColumnLayout {
            id: reply
            spacing: 0
            Layout.rightMargin: CmnCfg.smallMargin

            Label {
                id: opLabel
                text: !replyPreview.isDangling ? contactsModel.nameById(
                                                     replyPreview.author) : ""
                font.bold: true
                Layout.topMargin: CmnCfg.margin / 2
                Layout.bottomMargin: 0
                Layout.leftMargin: CmnCfg.smallMargin
                Layout.rightMargin: CmnCfg.smallMargin
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
                Layout.leftMargin: CmnCfg.smallMargin
                Layout.bottomMargin: CmnCfg.smallPadding
                Layout.topMargin: 0
                Layout.rightMargin: CmnCfg.smallMargin
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
