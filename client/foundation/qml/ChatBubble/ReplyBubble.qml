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
    property color opColor: CmnCfg.avatarColors[herald.users.colorById(
                                                    opAuthor)]
    property string authorName: ""
    property color authorColor
    property var replyId
    property alias jumpHandler: jumpHandler
    property alias replyHighlightAnimation: replyHighlightAnimation
    property bool knownReply: replyType == 2
    property bool elided: false
    property bool expanded: false

    Component.onCompleted: wrapperCol.expanded = false

    spacing: 0

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: reply.implicitHeight
        color: CmnCfg.palette.medGrey
        Layout.margins: CmnCfg.smallMargin
        Layout.minimumWidth: reply.width

        Rectangle {
            id: verticalAccent
            visible: knownReply
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
            enabled: knownReply ? true : false
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
                text: knownReply ? herald.users.nameById(opAuthor) : ""
                font.bold: true
                Layout.margins: CmnCfg.smallMargin
                Layout.bottomMargin: 0

                Layout.preferredHeight: knownReply ? implicitHeight : 0
                color: opColor
            }

            TextMetrics {
                id: opBodyTextMetrics
                property string decoration: knownReply
                                            && opBody.length > 350 ? "..." : ""
                property string shortenedText: knownReply ? truncate_text(
                                                                opBody).slice(
                                                                0,
                                                                350) + decoration : "Original message not found"
                text: shortenedText
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

            Row {
                spacing: 2
                Layout.bottomMargin: CmnCfg.smallPadding
                Layout.leftMargin: CmnCfg.smallMargin
                Layout.rightMargin: CmnCfg.smallMargin
                Label {
                    id: replyTs
                    Layout.margins: CmnCfg.smallMargin
                    Layout.topMargin: 0
                    font.pixelSize: 10
                    text: replyType === 2 ? Utils.friendlyTimestamp(
                                                opInsertionTime) : ""
                    color: CmnCfg.palette.darkGrey
                }

                Button {
                    id: clock
                    icon.source: opExpirationTime
                                 !== undefined ? "qrc:/countdown-icon-temp.svg" : ""
                    icon.height: 16
                    icon.width: 16
                    icon.color: "grey"
                    padding: 0
                    background: Item {}
                    anchors.verticalCenter: replyTs.verticalCenter
                }
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
    ElideHandler {}

    StandardStamps {}
}
