import QtQuick 2.0
import QtQuick.Controls 2.12

Row {
    spacing: CmnCfg.microMargin

    height: 20
    Repeater {
        id: emojiRepeater
        model: JSON.parse(messageModelData.reactions)
        property bool outboundReact

        delegate: Button {
            property var emojiModel: emojiRepeater.model
            property bool outboundReact
            visible: emojiModel[index]["reactionaries"].length !== 0
            font.pixelSize: 12
            font.family: CmnCfg.chatFont.name
            Component.onCompleted: {
                outboundReact = emojiModel[index]["reactionaries"].filter(
                            function (reactionary) {
                                return reactionary === Herald.config.configId
                            }).length === 1
            }

            padding: outboundReact ? CmnCfg.microMargin / 2 : 0
            topPadding: CmnCfg.microMargin / 2
            id: emojiText
            text: emojiModel[index]["content"] + " " + emojiModel[index]["reactionaries"].length
            background: Rectangle {
                border.width: outboundReact ? 1 : 0
                border.color: CmnCfg.palette.offBlack
                color: outboundReact ? CmnCfg.palette.lightGrey : "transparent"
            }
            onClicked: {
                if (outboundReact) {
                    return ownedConversation.removeReaction(
                                bubbleActual.bubbleIndex,
                                emojiModel[index]["content"])
                }
                return ownedConversation.addReaction(
                            bubbleActual.bubbleIndex,
                            emojiModel[index]["content"])
            }
        }
    }
}
