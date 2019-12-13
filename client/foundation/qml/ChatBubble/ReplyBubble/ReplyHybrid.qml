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
    property string fileCount

    spacing: 0

    Component.onCompleted: {
        JS.parseDocs(nameMetrics, modelData, fileSize, fileCount)
        JS.parseMedia(modelData, imageClip)
    }

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: replyWrapperCol.height
        color: CmnCfg.palette.medGrey
        Layout.margins: CmnCfg.smallMargin
        Layout.minimumWidth: 150
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

        //wraps attachment content and op message body
        ColumnLayout {
            id: replyWrapperCol
            spacing: 0

            //wraps op label + op doc clip + op media clip
            Item {
                Layout.preferredWidth: reply.width
                Layout.preferredHeight: 80

                //wraps op label + op doc clip
                ColumnLayout {
                    id: fileLabelWrapper
                    anchors.left: parent.left
                    Label {
                        id: opLabel
                        text: knownReply ? Herald.users.nameById(
                                               modelData.opAuthor) : ""
                        font.bold: true
                        Layout.margins: CmnCfg.smallMargin
                        Layout.bottomMargin: 0
                        Layout.preferredHeight: knownReply ? implicitHeight : 0
                        color: opColor
                    }

                    //wraps doc clip
                    Item {
                        id: fileClip
                        Layout.topMargin: CmnCfg.smallMargin
                        Layout.leftMargin: CmnCfg.smallMargin
                        Layout.preferredHeight: fileIcon.height
                        Image {
                            id: fileIcon
                            anchors.left: parent.left
                            anchors.verticalCenter: parent.verticalCenter
                            source: "qrc:/file-icon.svg"
                            height: 20
                            width: height
                        }

                        TextMetrics {
                            id: nameMetrics
                            elide: Text.ElideMiddle
                            elideWidth: reply.width - fileSize.width - 40 - CmnCfg.smallMargin * 2
                        }

                        Text {
                            id: fileName
                            anchors.left: fileIcon.right
                            anchors.leftMargin: CmnCfg.smallMargin
                            anchors.verticalCenter: parent.verticalCenter
                            color: CmnCfg.palette.black
                            text: nameMetrics.elidedText
                            font.family: CmnCfg.chatFont.name
                            font.pixelSize: 13
                            font.weight: Font.Medium
                        }

                        Text {
                            id: fileSize
                            anchors.left: fileName.right
                            anchors.leftMargin: CmnCfg.smallMargin
                            anchors.verticalCenter: parent.verticalCenter
                            font.family: CmnCfg.chatFont.name
                            font.pixelSize: 10
                            font.weight: Font.Light
                            color: CmnCfg.palette.darkGrey
                        }
                    }

                    //+ n more file count
                    Text {
                        id: fileSurplus
                        Layout.leftMargin: CmnCfg.smallMargin
                        Layout.topMargin: -CmnCfg.smallMargin / 2
                        visible: fileCount > 0
                        text: "+ " + fileCount + qsTr(" more")
                        font.weight: Font.Light
                        font.family: CmnCfg.chatFont.name
                        color: CmnCfg.palette.darkGrey
                        font.pixelSize: 13
                    }
                }

                //op image clip
                ReplyImageClip {
                    id: imageClip
                    anchors.topMargin: CmnCfg.smallMargin
                    anchors.top: parent.top
                    anchors.right: parent.right
                }
            }

            //op message body + timestamp
            ColumnLayout {
                id: reply
                spacing: 0
                Layout.topMargin: 0
                Layout.rightMargin: CmnCfg.smallMargin
                Layout.maximumWidth: bubbleRoot.imageAttach ? 300 : bubbleRoot.maxWidth
                Layout.minimumWidth: bubbleRoot.imageAttach ? 300 : Math.max(
                                                                  300,
                                                                  messageBody.width)
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
        }
    }
}
