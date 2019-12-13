import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../js/utils.mjs" as Utils
import "./js/utils.js" as JS
import QtQuick 2.13
import QtGraphicalEffects 1.12
import "../"

ColumnLayout {
    id: wrapperCol

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    modelData.opAuthor)]
    property var replyId
    property bool knownReply: modelData.replyType === 2
    property string replyBody: knownReply ? modelData.opBody : ""
    property var modelData

    spacing: 0

    Component.onCompleted: {
        if (modelData.opMediaAttachments.length === 0) {
            return
        }

        imageClipLoader.sourceComponent = imageClipComponent
        JS.parseMedia(modelData, imageClipLoader.item)
    }

    Rectangle {
        id: replyWrapper
        color: CmnCfg.palette.medGrey

        Layout.margins: CmnCfg.smallMargin
        Layout.preferredHeight: replyWrapperCol.height
        Layout.preferredWidth: replyWrapperCol.width

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
            anchors.centerIn: replyWrapperCol
            width: replyWrapperCol.width
            height: replyWrapperCol.height

            z: CmnCfg.overlayZ
            enabled: knownReply ? true : false

            onClicked: JS.jumpHandler(replyId, ownedConversation, convWindow)
        }

        ColumnLayout {
            id: replyWrapperCol

            RowLayout {
                id: replyRow
                height: reply.implicitHeight
                Layout.maximumWidth: bubbleRoot.imageAttach ? 300 : bubbleRoot.maxWidth
                Layout.minimumWidth: bubbleRoot.imageAttach ? 300 : messageBody.width
                clip: true

                ColumnLayout {
                    id: reply
                    spacing: 0
                    Layout.rightMargin: CmnCfg.smallMargin

                    Label {
                        id: opLabel
                        text: knownReply ? Herald.users.nameById(
                                               modelData.opAuthor) : ""
                        color: opColor

                        font.bold: true
                        Layout.margins: CmnCfg.smallMargin
                        Layout.bottomMargin: 0
                        Layout.topMargin: CmnCfg.smallMargin
                        Layout.preferredHeight: knownReply ? implicitHeight : 0
                    }

                    TextMetrics {
                        id: opBodyTextMetrics
                        property string shortenedText: knownReply ? modelData.opBody : qsTr(
                                                                        "Original message not found")
                        text: shortenedText
                        elideWidth: maxWidth * 2
                        elide: Text.ElideRight
                    }

                    StandardTextEdit {
                        id: replyBody
                        text: opBodyTextMetrics.elidedText
                        Layout.fillWidth: true
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

                Component {
                    id: imageClipComponent
                    ReplyImageClip {}
                }

                Loader {
                    id: imageClipLoader
                    Layout.margins: CmnCfg.smallMargin
                    Layout.leftMargin: 0
                }
            }
        }
    }
}
