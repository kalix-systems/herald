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
    property color authorColor
    property var replyId
    property alias jumpHandler: jumpHandler
    property alias replyHighlightAnimation: replyHighlightAnimation

    spacing: 0

    MessagePreview {
        id: replyPreview
        messageId: replyId === undefined ? null : replyId
    }

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: reply.implicitHeight
        color: CmnCfg.palette.sideBarHighlightColor
        Layout.margins: CmnCfg.smallMargin
        Layout.minimumWidth: reply.width

        Rectangle {
            id: verticalAccent
            visible: !replyPreview.isDangling
            anchors.right: !outbound ? replyWrapper.left : undefined
            anchors.left: outbound ? replyWrapper.right : undefined
            height: replyWrapper.height
            width: CmnCfg.smallMargin / 4
            color: opColor
        }

        MouseArea {
            id: jumpHandler
            anchors.centerIn: reply
            width: reply.width
            height: reply.height
            z: CmnCfg.overlayZ
            enabled: !replyPreview.isDangling ? true : false
        }

        NumberAnimation {
            id: replyHighlightAnimation
            property: "opacity"
            from: 1.0
            to: 0.0
            duration: 600
            easing.type: Easing.InCubic
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
                Layout.margins: CmnCfg.smallMargin
                Layout.bottomMargin: 0

                Layout.preferredHeight: !replyPreview.isDangling ? implicitHeight : 0
                color: opColor
            }

            TextMetrics {
                id: opBodyTextMetrics
                property string shortenedText: truncate_text(
                                                   replyPreview.body).slice(
                                                   0, 350) + decoration
                readonly property string decoration: replyPreview.body.length > 350 ? "..." : ""

                text: !replyPreview.isDangling ? shortenedText : "Original message not found"
                elideWidth: maxWidth * 3
                elide: Text.ElideRight

                function truncate_text(body) {
                    var bodyLines = body.split("\n")
                    if (bodyLines.length > 3) {
                        return bodyLines.slice(0, 3).join("\n")
                    } else {
                        return body
                    }
                }
            }

            StandardTextEdit {
                id: replyBody
                text: opBodyTextMetrics.elidedText
                Layout.minimumWidth: messageBody.width
            }

            Label {
                Layout.margins: CmnCfg.smallMargin
                Layout.topMargin: 0
                font.pixelSize: 10
                text: !replyPreview.isDangling ? Utils.friendlyTimestamp(
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

    StandardStamps {
    }
}
