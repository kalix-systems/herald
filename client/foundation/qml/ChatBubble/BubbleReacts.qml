import QtQuick 2.0
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtGraphicalEffects 1.13

Flow {
    spacing: CmnCfg.microMargin
    width: bubbleRoot.defaultWidth - avatar.width - CmnCfg.smallMargin * 2
    height: implicitHeight
    Repeater {
        id: emojiRepeater
        model: JSON.parse(messageModelData.reactions)
        property bool outboundReact

        delegate: Button {
            id: emojiText
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
            MouseArea {
                enabled: !bubbleRoot.moreInfo
                anchors.fill: parent
                hoverEnabled: true
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
                cursorShape: bubbleRoot.moreInfo ? Qt.ArrowCursor : Qt.PointingHandCursor
            }

            padding: outboundReact ? CmnCfg.microMargin / 2 : 0
            topPadding: CmnCfg.microMargin / 2

            contentItem: Row {
                spacing: CmnCfg.microMargin
                Label {
                    id: emoji
                    text: emojiModel[index]["content"]
                }
                Label {
                    id: numLabel
                    text: emojiModel[index]["reactionaries"].length
                    font.family: CmnCfg.chatFont.name
                    color: CmnCfg.palette.offBlack
                    font.pixelSize: 11
                    anchors.verticalCenter: emoji.verticalCenter
                }
            }

            background: Rectangle {

                border.width: outboundReact ? 1 : 0
                border.color: CmnCfg.palette.offBlack
                color: CmnCfg.palette.lightGrey
            }
        }
    }
}
