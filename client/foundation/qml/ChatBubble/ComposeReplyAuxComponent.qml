import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports" as Imports
import "qrc:/imports/ChatBubble/ReplyBubble"
import "qrc:/imports/ChatBubble" as ChatBubble

Rectangle {
    id: wrapper
    height: Math.max(wrapperCol.height, 20)
    color: CmnCfg.palette.medGrey

    //pass in messages.builder on desktop and mobile
    property var builderData
    property var auxData: JSON.parse(builderData.opAuxContent)

    property string authorName: Herald.users.nameById(builderData.opAuthor)
    property string friendlyTimestamp: Utils.friendlyTimestamp(
                                           builderData.opTime)
    property var maxWidth: wrapper.width - CmnCfg.smallMargin

    Connections {
        target: appRoot.globalTimer
        onRefreshTime: friendlyTimestamp = Utils.friendlyTimestamp(
                           builderData.opTime)
    }
    Connections {
        target: builderData
        onOpIdChanged: if (builderData.isReply) {
                           friendlyTimestamp = Utils.friendlyTimestamp(
                                       builderData.opTime)
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
        color: CmnCfg.palette.darkGrey
        anchors.left: parent.left
    }

    Column {
        id: wrapperCol
        anchors.left: accent.right
        width: wrapper.width

        spacing: CmnCfg.smallMargin
        padding: CmnCfg.smallMargin
        Text {
            text: friendlyTimestamp
            font.family: CmnCfg.chatFont.name
            font.italic: true
            font.pixelSize: 12
            color: CmnCfg.palette.darkGrey
            elide: Text.ElideRight
        }

        Text {
            id: actionText
            text: authorName + Utils.auxString(auxData.code, auxData.content)
            font.family: CmnCfg.chatFont.name
            font.italic: true
            elide: Text.ElideRight
        }

        Loader {
            id: imageClip
            active: auxData.code === 3
            height: active ? item.height : 0
            sourceComponent: ReplyImageClip {
                imageSource: "file:" + auxData.content
            }
        }
    }
}
