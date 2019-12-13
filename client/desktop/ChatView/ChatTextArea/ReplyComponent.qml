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
    color: CmnCfg.palette.medGrey
    width: parent.width
    height: Math.max(textCol.height, 20)

    property color startColor
    property string opText: parent.opText
    property string opName: parent.opName

    function loadMedia() {
        const media = ownedConversation.builder.opMediaAttachments.length
                    === 0 ? "" : JSON.parse(
                                ownedConversation.builder.opMediaAttachments)
        switch (media.length) {
        case 0:
            break
        default:
            imageClipLoader.sourceComponent = imageClipComponent
            imageClipLoader.item.imageSource = "file:" + media[0].path
            imageClipLoader.item.count = media.length - 1
            imageClipLoader.item.aspectRatio = media[0].width / media[0].height
        }
    }

    Component.onCompleted: loadMedia()
    Connections {
        target: ownedConversation.builder
        onOpIdChanged: if (ownedConversation.builder.isReply) {
                           loadMedia()
                       }
    }

    Rectangle {
        id: verticalAccent
        anchors.right: wrapper.left
        height: wrapper.height
        width: CmnCfg.smallMargin / 4
        color: CmnCfg.palette.avatarColors[Herald.users.colorById(
                                               ownedConversation.builder.opAuthor)]
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
        onClicked: ownedConversation.builder.clearReply()
    }

    ColumnLayout {
        RowLayout {
            Layout.preferredWidth: wrapper.width - CmnCfg.smallMargin
            Layout.maximumWidth: wrapper.width - CmnCfg.smallMargin
            clip: true
            ColumnLayout {
                id: textCol
                Label {
                    id: sender
                    text: Herald.users.nameById(
                              ownedConversation.builder.opAuthor)
                    Layout.leftMargin: CmnCfg.smallMargin
                    Layout.rightMargin: CmnCfg.smallMargin
                    Layout.bottomMargin: CmnCfg.margin / 2
                    Layout.topMargin: CmnCfg.margin / 2
                    Layout.preferredHeight: CmnCfg.smallMargin
                    font.bold: true
                    color: CmnCfg.palette.avatarColors[Herald.users.colorById(
                                                           ownedConversation.builder.opAuthor)]
                }

                TextMetrics {
                    id: opTextMetrics
                    text: ownedConversation.builder.opBody
                    elideWidth: (wrapper.width - CmnCfg.smallMargin) * 2
                    elide: Text.ElideRight
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
