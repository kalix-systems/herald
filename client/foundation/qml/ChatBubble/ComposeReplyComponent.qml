import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports" as Imports
import "qrc:/imports/ChatBubble/ReplyBubble"
import "qrc:/imports/ChatBubble" as ChatBubble
import "qrc:/imports/ChatBubble/ReplyBubble/dyn"

Rectangle {
    id: wrapper
    height: Math.max(wrapperRow.height + label.height, 20)
    color: CmnCfg.palette.medGrey

    //pass in messages.builder on desktop and mobile
    property var builderData
    property bool outboundReply: Herald.config.configId === builderData.opAuthor

    property color authorColor: CmnCfg.palette.avatarColors[Herald.users.colorById(
                                                                builderData.opAuthor)]
    property string authorName: outboundReply ? Herald.config.name : Herald.users.nameById(
                                                    builderData.opAuthor)
    property string friendlyTimestamp: Utils.friendlyTimestamp(
                                           builderData.opTime)
    property var maxWidth: wrapper.width - CmnCfg.smallMargin
    property var emptyImage: builderData.opBody === ""
                             && (builderData.opMediaAttachments.length !== 0
                                 && builderData.opDocAttachments.length === 0)

    Connections {
        target: appRoot.globalTimer
        onRefreshTime: friendlyTimestamp = Utils.friendlyTimestamp(
                           builderData.opTime)
    }
    Connections {
        target: builderData
        onOpIdChanged: if (builderData.isReply) {
                           loadMedia()
                           loadDocs()
                           friendlyTimestamp = Utils.friendlyTimestamp(
                                       builderData.opTime)
                       }
    }

    Component.onCompleted: {
        loadMedia()
        loadDocs()
    }

    function loadMedia() {
        const media = builderData.opMediaAttachments.length
                    === 0 ? "" : JSON.parse(builderData.opMediaAttachments)
        switch (media.length) {
        case 0:
            imageClipLoader.sourceComponent = undefined
            break
        default:
            imageClipLoader.sourceComponent = imageClipComponent
            imageClipLoader.item.imageSource = "file:" + media.items[0].path
            imageClipLoader.item.count
                    = (media.num_more === 0) ? media.items.length - 1 : media.items.length
                                               - 1 + media.num_more
        }
    }

    function loadDocs() {
        const doc = builderData.opDocAttachments.length === 0 ? "" : JSON.parse(
                                                                    builderData.opDocAttachments)
        switch (doc.length) {
        case 0:
            fileClipLoader.sourceComponent = undefined
            break
        default:
            fileClipLoader.sourceComponent = fileClipComponent
            fileClipLoader.item.nameMetrics.text = doc.items[0].name
            fileClipLoader.item.fileSize.text = Utils.friendlyFileSize(
                        doc.items[0].size)
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
        fill: CmnCfg.palette.black
        onClicked: builderData.clearReply()
    }

    Rectangle {
        id: accent
        anchors.top: parent.top
        anchors.bottom: parent.bottom

        width: CmnCfg.accentBarWidth
        color: authorColor
        anchors.left: parent.left
    }

    ChatBubble.BubbleLabel {
        id: label
        topPadding: CmnCfg.smallMargin
        anchors.left: accent.right
        anchors.leftMargin: CmnCfg.smallMargin

        timestamp: friendlyTimestamp
        name: authorName
    }

    Row {
        width: maxWidth
        id: wrapperRow
        clip: true
        padding: CmnCfg.smallMargin
        leftPadding: 0
        spacing: CmnCfg.smallMargin

        anchors.top: label.bottom
        anchors.left: accent.right

        anchors.leftMargin: CmnCfg.smallMargin
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
                    ReplyFileClip {
                        elideWidth: maxWidth
                    }
                }
            }

            TextEdit {

                TextMetrics {
                    id: opTextMetrics
                    text: builderData.opBody
                    elideWidth: maxWidth * 2
                    elide: Text.ElideRight
                }

                text: emptyImage ? "<i>Media message</i>" : opTextMetrics.elidedText
                Layout.rightMargin: CmnCfg.smallMargin
                Layout.alignment: Qt.AlignLeft
                Layout.fillWidth: true

                wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                selectByMouse: true
                selectByKeyboard: true
                readOnly: true
                color: CmnCfg.palette.black
                textFormat: Text.RichText
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
