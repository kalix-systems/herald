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
ListView {
    id: conversationList

    clip: true
    currentIndex: -1
    boundsBehavior: Flickable.StopAtBounds


    Connections {
        target: networkHandle
        onNewConversationChanged: {
            if (networkHandle.newConversation !== 0) {
                conversationsModel.refresh(networkHandle.nextNewConversation())
            }
        }
    }

    ScrollBar.vertical: ScrollBar {
    }

    delegate: Item {
        id: conversationItem

        readonly property var conversationIdProxy: conversationId
        property bool isPairwise: pairwise

        property Messages messageModel: Messages {
            conversationId: conversationIdProxy
        }

        property var childChatView: Component {
            CV.ChatView {
                conversationAvatar: conversationItemAvatar
                ownedConversation: messageModel
            }
        }

        Users {
            id: convoItemMembers
            conversationId: conversationIdProxy
        }

        visible: matched
        // This ternary is okay, types are enforced by QML
        height: visible ? 55 : 0
        width: parent.width

        Rectangle {
            id: bgBox
            readonly property color focusColor: QmlCfg.palette.tertiaryColor
            readonly property color hoverColor: QmlCfg.palette.secondaryColor
            readonly property color defaultColor: QmlCfg.palette.mainColor

            anchors.fill: parent

            Common.Divider {
                color: QmlCfg.palette.secondaryColor
                anchor: parent.bottom
                height: 2
            }

            Common.Avatar {
                id: conversationItemAvatar
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
                        color: hoverColor
                    }
                },
                State {
                    when: conversationItem.focus
                    name: "selected"
                    PropertyChanges {
                        target: bgBox
                        color: focusColor
                    }
                }
            ]

            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: 10
                anchors.fill: parent
                // ToDo: remove the imperative state transitions if it does not break
                // anything. they are now handled with `when` bindings
                //                onEntered: parent.state = "hovering"
                //                onExited: parent.state = ""
                onClicked: {
                    chatView.sourceComponent = childChatView
                    conversationList.currentIndex = index
                }

                // ternary is okay here, type enforced by QML
                //onReleased: parent.state = containsMouse ? "hovering" : ""
            }
            // ternary is okay here, type enforced by QML
            // color: conversationItem.focus ? focusColor : defaultColor
        }
    }
}
