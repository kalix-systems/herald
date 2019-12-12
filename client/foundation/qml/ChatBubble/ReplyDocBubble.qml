import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../js/utils.mjs" as Utils
import QtQuick 2.13
import QtGraphicalEffects 1.12

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
        const doc = JSON.parse(modelData.opDocAttachments)

        nameMetrics.text = doc[0].name
        fileSize.text = Utils.friendlyFileSize(doc[0].size)
        fileCount = doc.length - 1

        if (modelData.opMediaAttachments.length === 0)
            return

        const media = JSON.parse(modelData.opMediaAttachments)

        if (media.length === 0)
            return

        imageClipLoader.sourceComponent = imageClipComponent
        imageClipLoader.item.imageSource = "file:" + media[0].path
        imageClipLoader.item.count = media.length - 1
        imageClipLoader.item.aspectRatio = media[0].width / media[0].height
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
            id: replyWrapperCol
            Row {
                ColumnLayout {
                    id: fileWrapper
                    Label {
                        id: opLabel
                        text: knownReply ? Herald.users.nameById(
                                               modelData.opAuthor) : ""
                        font.bold: true
                        Layout.margins: CmnCfg.smallMargin
                        Layout.bottomMargin: 0
                        Layout.topMargin: CmnCfg.smallMargin
                        Layout.preferredHeight: knownReply ? implicitHeight : 0
                        color: opColor
                    }

                    Item {
                        id: attachmentRow
                        Layout.alignment: Qt.AlignTop
                        Layout.preferredWidth: replyWrapper.width - 80
                        Layout.preferredHeight: fileCount > 0 ? fileIcon.height + CmnCfg.smallMargin
                                                                * 3 : fileIcon.height
                        Layout.topMargin: 0
                        Layout.leftMargin: CmnCfg.smallMargin
                        Image {
                            id: fileIcon
                            anchors.left: parent.left
                            anchors.verticalCenter: parent.verticalCenter
                            source: "qrc:/file-icon.svg"
                            height: 20
                            width: height

                            Text {
                                anchors.top: parent.bottom
                                visible: fileCount > 0
                                text: "+ " + fileCount + " more"
                                font.weight: Font.Light
                                font.family: CmnCfg.chatFont.name
                                color: CmnCfg.palette.darkGrey
                                font.pixelSize: 13
                            }
                        }

                        TextMetrics {
                            id: nameMetrics
                            elide: Text.ElideMiddle
                            elideWidth: reply.width - imageClipLoader.size
                                        - fileSize.width - 40 - CmnCfg.smallMargin * 2
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
                }

                Loader {
                    property int size: item == undefined ? 16 : 80
                    id: imageClipLoader
                    anchors.top: parent.top
                    anchors.topMargin: CmnCfg.smallMargin
                }

                Component {
                    id: imageClipComponent
                    ReplyImageClip {}
                }
            }

            ColumnLayout {
                id: reply
                spacing: 0
                Layout.alignment: Qt.AlignTop
                Layout.rightMargin: CmnCfg.smallMargin
                Layout.maximumWidth: bubbleRoot.imageAttach ? 300 : bubbleRoot.maxWidth
                Layout.minimumWidth: bubbleRoot.imageAttach ? 300 : Math.max(
                                                                  300,
                                                                  messageBody.width)
                TextMetrics {
                    id: opBodyTextMetrics
                    property string decoration: replyBody > 350 ? "..." : ""
                    property string shortenedText: knownReply ? truncate_text(
                                                                    modelData.opBody).slice(
                                                                    0,
                                                                    350) + decoration : qsTr(
                                                                    "Original message not found")
                    text: shortenedText
                    elideWidth: maxWidth * 2
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
