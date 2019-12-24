import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.12
import "qrc:/common" as Common
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports" as Imports
import "qrc:/imports/ChatBubble/ReplyBubble"

Rectangle {
    id: wrapper
    color: CmnCfg.palette.lightGrey
    border.color: CmnCfg.palette.black
    border.width: 1
    height: Math.max(wrapperRow.height + label.height, 20)

    Connections {
        target: appRoot.globalTimer
        onRefreshTime: timestamp.text = Utils.friendlyTimestamp(
                           ownedConversation.builder.opTime)
    }

    Label {
        id: label
        anchors.top: parent.top
        anchors.left: parent.left
        width: parent.width
        font.family: CmnCfg.chatFont.name
        font.weight: Font.Bold
        padding: 2
        leftPadding: CmnCfg.microMargin

        color: CmnCfg.palette.white
        text: Herald.users.nameById(ownedConversation.builder.opAuthor)
        background: Rectangle {
            color: CmnCfg.palette.avatarColors[Herald.users.colorById(
                                                   ownedConversation.builder.opAuthor)]
            border.color: Qt.darker(color, 1.3)
            border.width: 1
        }
    }

    property color startColor
    property string opText: parent.opText
    property string opName: parent.opName

    function loadMedia() {
        const media = ownedConversation.builder.opMediaAttachments.length
                    === 0 ? "" : JSON.parse(
                                ownedConversation.builder.opMediaAttachments)
        switch (media.length) {
        case 0:
            imageClipLoader.sourceComponent = undefined
            break
        default:
            imageClipLoader.sourceComponent = imageClipComponent
            imageClipLoader.item.imageSource = "file:" + media[0].path
            imageClipLoader.item.count = media.length - 1
            imageClipLoader.item.aspectRatio = media[0].width / media[0].height
        }
    }

    function loadDocs() {
        const doc = ownedConversation.builder.opDocAttachments.length
                  === 0 ? "" : JSON.parse(
                              ownedConversation.builder.opDocAttachments)
        switch (doc.length) {
        case 0:
            fileClipLoader.sourceComponent = undefined
            break
        default:
            fileClipLoader.sourceComponent = fileClipComponent
            fileClipLoader.item.nameMetrics = doc[0].name
            fileClipLoader.item.fileSize = Utils.friendlyFileSize(doc[0].size)
        }
    }

    Component.onCompleted: {
        loadMedia()
        loadDocs()
    }
    Connections {
        target: ownedConversation.builder
        onOpIdChanged: if (ownedConversation.builder.isReply) {
                           loadMedia()
                           loadDocs()
                       }
    }

    Imports.IconButton {
        id: exitButton
        anchors {
            right: parent.right
            top: parent.top
        }
        source: "qrc:/x-icon.svg"
        scale: 0.8
        fill: CmnCfg.palette.white
        onClicked: ownedConversation.builder.clearReply()
    }

    Row {
        width: wrapper.width - CmnCfg.smallMargin
        id: wrapperRow
        clip: true
        padding: CmnCfg.smallMargin
        spacing: CmnCfg.smallMargin

        anchors.top: label.bottom
        ColumnLayout {
            id: textCol
            width: parent.width - imageClipLoader.width - CmnCfg.smallMargin * 2
            spacing: CmnCfg.smallMargin

            Loader {
                id: fileClipLoader
                Layout.preferredHeight: item ? item.height : 0
                Layout.preferredWidth: item ? item.width : 0
                Component {
                    id: fileClipComponent
                    FileClip {}
                }
            }

            TextEdit {

                TextMetrics {
                    id: opTextMetrics
                    text: ownedConversation.builder.opBody
                    elideWidth: (wrapper.width - CmnCfg.smallMargin) * 2
                    elide: Text.ElideRight
                }

                text: opTextMetrics.elidedText
                Layout.rightMargin: CmnCfg.smallMargin
                Layout.alignment: Qt.AlignLeft
                Layout.fillWidth: true

                wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                selectByMouse: true
                selectByKeyboard: true
                readOnly: true
                color: CmnCfg.palette.black
            }

            Label {
                id: timestamp
                Layout.topMargin: 0
                Layout.rightMargin: CmnCfg.smallMargin
                font.pixelSize: 10
                text: Utils.friendlyTimestamp(ownedConversation.builder.opTime)
                color: CmnCfg.palette.darkGrey
            }
        }

        Loader {
            id: imageClipLoader
            height: item ? 64 : 0
            width: item ? 64 : 0
            Component {
                id: imageClipComponent
                ReplyImageClip {}
            }
        }
    }
}
