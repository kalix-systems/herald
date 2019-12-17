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
    color: CmnCfg.palette.white
    border.color: CmnCfg.palette.black
    border.width: 1
    width: parent.width
    height: Math.max(
                textCol.implicitHeight + label.height + CmnCfg.smallMargin, 20)

    Label {
        id: label
        anchors.top: parent.top
        anchors.left: parent.left
        width: parent.width
        font.family: CmnCfg.chatFont.name
        font.weight: Font.Bold
        padding: CmnCfg.smallMargin / 4
        leftPadding: CmnCfg.smallMargin / 2

        color: CmnCfg.palette.white
        text: Herald.users.nameById(ownedConversation.builder.opAuthor)
        background: Rectangle {
            color: CmnCfg.palette.avatarColors[Herald.users.colorById(
                                                   ownedConversation.builder.opAuthor)]
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
            fileClipLoader.item.fileSize = doc[0].size
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

    Imports.ButtonForm {
        id: exitButton
        anchors {
            //  margins: CmnCfg.smallMargin
            right: parent.right
            top: parent.top
        }
        source: "qrc:/x-icon.svg"
        scale: 0.8
        fill: CmnCfg.palette.white
        onClicked: ownedConversation.builder.clearReply()
    }

    ColumnLayout {
        anchors.top: label.bottom
        RowLayout {
            Layout.preferredWidth: wrapper.width - CmnCfg.smallMargin
            Layout.maximumWidth: wrapper.width - CmnCfg.smallMargin
            clip: true
            ColumnLayout {
                id: textCol

                TextMetrics {
                    id: opTextMetrics
                    text: ownedConversation.builder.opBody
                    elideWidth: (wrapper.width - CmnCfg.smallMargin) * 2
                    elide: Text.ElideRight
                }

                Loader {
                    id: fileClipLoader
                    height: item ? item.height : 0
                    onHeightChanged: print(height)
                }

                Component {
                    id: fileClipComponent
                    FileClip {}
                }

                TextEdit {
                    text: opTextMetrics.elidedText
                    Layout.topMargin: CmnCfg.margin / 2
                    Layout.leftMargin: CmnCfg.smallMargin
                    Layout.rightMargin: CmnCfg.smallMargin
                    Layout.bottomMargin: CmnCfg.smallPadding
                    Layout.alignment: Qt.AlignLeft
                    Layout.fillWidth: true

                    wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                    selectByMouse: true
                    selectByKeyboard: true
                    readOnly: true
                    color: CmnCfg.palette.black
                }

                Label {
                    Layout.leftMargin: CmnCfg.smallMargin
                    Layout.bottomMargin: CmnCfg.smallPadding
                    Layout.topMargin: 0
                    Layout.rightMargin: CmnCfg.smallMargin
                    font.pixelSize: 10
                    text: Utils.friendlyTimestamp(
                              ownedConversation.builder.opTime)
                    id: timestamp
                    color: CmnCfg.palette.darkGrey
                }
            }

            Component {
                id: imageClipComponent
                ReplyImageClip {}
            }

            Loader {
                Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                Layout.rightMargin: CmnCfg.largeMargin
                id: imageClipLoader
            }
        }
    }
}
