import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../js/utils.mjs" as Utils
import QtQuick 2.13

ColumnLayout {
    id: wrapperCol

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property color opColor: CmnCfg.avatarColors[herald.users.colorById(
                                                    modelData.opAuthor)]
    property var replyId
    property alias jumpHandler: jumpHandler
    property bool knownReply: modelData.replyType === 2
    property string replyBody: knownReply ? modelData.opBody : ""
    property var modelData

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

            onClicked: {
                const msgIndex = ownedConversation.indexById(replyId)

                if (msgIndex < 0)
                    return

                const window = convWindow

                window.positionViewAtIndex(msgIndex, ListView.Center)
                window.highlightAnimation.target = window.itemAtIndex(
                            msgIndex).highlight
                window.highlightAnimation.start()
            }
        }

        ColumnLayout {
            id: reply
            spacing: 0
            Layout.rightMargin: CmnCfg.smallMargin

            Label {
                id: opLabel
                text: knownReply ? herald.users.nameById(
                                       modelData.opAuthor) : ""
                font.bold: true
                Layout.margins: CmnCfg.smallMargin
                Layout.bottomMargin: 0

                Layout.preferredHeight: knownReply ? implicitHeight : 0
                color: opColor
            }

            TextMetrics {
                id: opBodyTextMetrics
                property string decoration: replyBody > 350 ? "..." : ""
                property string shortenedText: knownReply ? truncate_text(
                                                                modelData.opBody).slice(
                                                                0,
                                                                350) + decoration : "Original message not found"
                text: shortenedText
                elideWidth: maxWidth * 3
                elide: Text.ElideRight

                function truncate_text(body) {
                    const bodyLines = body.split("\n")
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
                Layout.minimumWidth: bubbleRoot.imageAttach ? 300 : messageBody.width
                Layout.maximumWidth: bubbleRoot.imageAttach ? 300 : bubbleRoot.maxWidth
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
                    text: modelData.replyType === 2 ? Utils.friendlyTimestamp(
                                                          modelData.opInsertionTime) : ""
                    color: CmnCfg.palette.darkGrey
                }

                Button {
                    id: clock
                    icon.source: modelData.opExpirationTime
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
}
