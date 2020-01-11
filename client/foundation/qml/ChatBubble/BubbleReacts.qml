import QtQuick 2.0
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
            property string reactions: ""
            visible: emojiModel[index]["reactionaries"].length !== 0
            font.pixelSize: 12
            font.family: CmnCfg.chatFont.name
            Component.onCompleted: {
                outboundReact = emojiModel[index]["reactionaries"].filter(
                            function (reactionary) {
                                return reactionary === Herald.config.configId
                            }).length === 1

                for (var reactionary in emojiModel[index]["reactionaries"]) {
                    reactions += (Herald.users.nameById(
                                      emojiModel[index]["reactionaries"][reactionary]) + ", ")
                }
                reactions = reactions.slice(0, reactions.length - 2)
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
                    contentItem: GridLayout {
                        Label {
                            text: emojiText.reactions !== undefined ? emojiText.reactions : ""
                            Layout.maximumWidth: 100
                            Layout.maximumHeight: 50
                            wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                            padding: 2
                            font.pixelSize: CmnCfg.minorTextSize
                            font.weight: Font.Medium
                            color: CmnCfg.palette.white
                            font.family: CmnCfg.chatFont.name
                        }
                    }
                }
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
