import QtQuick 2.14
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.3

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
            font.pixelSize: CmnCfg.chatTextSize
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
                propagateComposedEvents: true
                onEntered: bubbleActual.hoverHighlight = true
                onExited: if (!bubbleActual.hitbox.containsMouse) {
                              bubbleActual.hoverHighlight = false
                          }

                ToolTip {
                    delay: 500

                    visible: parent.containsMouse
                    y: -height
                    background: Rectangle {
                        color: CmnCfg.palette.offBlack
                        border.width: 0
                    }
                    padding: 2
                    contentItem: Column {
                        Repeater {
                            id: repeater
                            model: emojiText.emojiModel[index]["reactionaries"]
                            delegate: Label {
                                property var reactData: repeater.model
                                text: UserMap.get(reactData[index]).name
                                font.pixelSize: CmnCfg.minorTextSize
                                font.weight: Font.Medium
                                color: CmnCfg.palette.white
                                font.family: CmnCfg.chatFont.name
                                padding: 2
                            }
                        }
                    }
                }
            }

            topPadding: CmnCfg.microMargin / 2
            padding: 1

            contentItem: Row {
                spacing: CmnCfg.microMargin
                Label {
                    id: emoji
                    text: emojiModel[index]["content"]
                    color: outboundReact ? CmnCfg.palette.white : CmnCfg.palette.black
                    font.pixelSize: CmnCfg.minorTextSize
                }
                Label {
                    id: numLabel
                    text: emojiModel[index]["reactionaries"].length
                    font.family: CmnCfg.chatFont.name
                    color: outboundReact ? CmnCfg.palette.white : CmnCfg.palette.offBlack

                    font.pixelSize: CmnCfg.minorTextSize
                    font.weight: Font.Medium
                    anchors.verticalCenter: emoji.verticalCenter
                }
            }

            background: Rectangle {

                border.width: outboundReact ? 0 : 1
                border.color: CmnCfg.palette.darkGrey
                color: outboundReact ? CmnCfg.palette.darkGrey : CmnCfg.palette.lightGrey
            }
        }
    }
}
