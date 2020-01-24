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
    property MessageBuilder builderData
    property var auxData: JSON.parse(builderData.opAuxContent)

    property string authorName: outboundReply ? UserMap.get(
                                                    Herald.config.configId).name : UserMap.get(
                                                    builderData.opAuthor).name

    property color authorColor: CmnCfg.palette.avatarColors[UserMap.get(
                                                                builderData.opAuthor).userColor]
    property string friendlyTimestamp: Utils.friendlyTimestamp(
                                           builderData.opTime)
    property var maxWidth: wrapper.width - CmnCfg.smallMargin

    property bool outboundReply: Herald.config.configId === builderData.opAuthor

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

    Column {
        id: wrapperCol
        anchors.left: parent.left
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
