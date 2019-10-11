import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "../common/utils.mjs" as Utils
import "../ChatView" as CV
import "./ContactView.mjs" as JS
import "popups" as Popups

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC

/// --- displays a list of conversations

// PAUL: this can be refactored into a filterable list component.
// we will need this for filtering raw text as well, lets not repeat code.
ListView {
    id: conversationList
    clip: true
    currentIndex: -1
    boundsBehavior: Flickable.StopAtBounds

    //PAUL: , lets write our own QML formatter so that this is a one liner
    ScrollBar.vertical: ScrollBar {
    }

    delegate: Item {
        id: conversationItem

        readonly property var conversationIdProxy: conversationId
        property bool isPairwise: pairwise

        property Messages messageModel: Messages {
            conversationId: conversationIdProxy
        }

        Connections {
            target: networkHandle
            onMsgDataChanged: messageModel.pollUpdate()
        }

        property var childChatView: Component {
            CV.ChatView {
                conversationAvatar: conversationItemAvatar
                ownedConversation: messageModel
            }
        }

        Members {
            id: convoItemMembers
            conversationId: conversationIdProxy
        }

        visible: matched
        height: visible ? 55 : 0
        width: parent.width

        //KAAVYA: this should be a factored out component
        // so it can be reused in contactview and convoview

        Rectangle {
            id: bgBox
            color: QmlCfg.palette.mainColor
            anchors.fill: parent

            Common.Divider {
                color: QmlCfg.palette.secondaryColor
                anchor: parent.bottom
                // PAUL: convert to device independent size this is magic.
                height: 2
            }

            Common.Avatar {
                id: conversationItemAvatar
                // PAUL: convert to device independent size this is magic.
                size: 45
                labeled: false
                labelGap: QmlCfg.smallMargin
                avatarLabel: Utils.unwrapOr(title, "unknown")
                colorHash: Utils.unwrapOr(color, 0)
                pfpUrl: Utils.safeStringOrDefault(picture)
            }

            ConversationLabel {
                anchors.left: conversationItemAvatar.right
                anchors.right: parent.right
                label: Utils.unwrapOr(title, "unknown")
                summaryText: JS.formatSummary(messageModel.lastAuthor,
                                              messageModel.lastBody)
            }

            states: [
                State {
                    when: hoverHandler.containsMouse
                    name: "hovering"
                    PropertyChanges {
                        target: bgBox
                        color: QmlCfg.palette.secondaryColor
                    }
                },
                State {
                    when: conversationItem.focus
                    name: "selected"
                    PropertyChanges {
                        target: bgBox
                        color: QmlCfg.palette.tertiaryColor
                    }
                }
            ]

            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: 10 // PAUL: unmagic all the z's
                anchors.fill: parent
                onClicked: {
                    chatView.sourceComponent = childChatView
                    conversationList.currentIndex = index
                }
            }
        }
    }
}
