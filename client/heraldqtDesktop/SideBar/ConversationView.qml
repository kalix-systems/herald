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

    //Connections {
    //    target: convModel
    //    onConversationIdChanged: {
    //        if (convModel.conversationId === undefined) {
    //            conversationList.currentIndex = -1
    //        }
    //    }
    //}

    ScrollBar.vertical: ScrollBar {
    }

    delegate: Item {
        id: conversationItem

        //GS : rexporting the contact avatar to global state is a backwards ref!
        property Item conversationAvatar: conversationAvatar
        property var conversationIdProxy: conversationId
        property bool isPairwise: pairwise
        property Messages messageModel: Messages {
            conversationId: conversationIdProxy
          }

        property var childChatView: Component {
            CV.ChatView {
              ownedConversation: messageModel
           }
        }


        Users {
            id: convoItemMembers
            conversationId: conversationIdProxy
        }

        // This ternary is okay, types are enforced by QML
        visible: matched
        height: visible ? 55 : 0
        width: parent.width

        /// NPB : This ought to be a mouse area with a hovered handler
        Rectangle {
            id: bgBox
            readonly property color focusColor: QmlCfg.palette.tertiaryColor
            readonly property color hoverColor: QmlCfg.palette.secondaryColor
            readonly property color defaultColor: QmlCfg.palette.mainColor

            Common.Divider {
                color: QmlCfg.palette.secondaryColor
                anchor: parent.bottom
                height: 2
            }

            Common.Avatar {
                id: conversationAvatar
                size: 45
                labeled: false
                labelGap: QmlCfg.smallMargin
                avatarLabel: Utils.unwrapOr(title, "unknown")
                colorHash: Utils.unwrapOr(color, 0)
                pfpUrl: Utils.safeStringOrDefault(picture)
            }

            ConversationLabel {
                anchors.left: conversationAvatar.right
                anchors.right: parent.right
                label: Utils.unwrapOr(title, "unknown")
                summaryText: JS.formatSummary(messageModel.lastAuthor, messageModel.lastBody)
            }

            anchors.fill: parent

            /// Note: can we use the highlight property here
            /// we can do this once contact deletion updates current item for listview properly
            states: [
                State {
                    name: "hovering"
                    PropertyChanges {
                        target: bgBox
                        color: hoverColor
                    }
                },
                State {
                    name: "focused"
                    PropertyChanges {
                        target: bgBox
                        color: focusColor
                    }
                }
            ]

            MouseArea {
                hoverEnabled: true
                z: 10
                anchors.fill: parent
                onEntered: parent.state = "hovering"
                onExited: parent.state = ""

                onClicked: {
                    chatView.sourceComponent = childChatView;
                    conversationList.currentIndex = index
                }

                // ternary is okay here, type enforced by QML
                onReleased: parent.state = containsMouse ? "hovering" : ""
            }
            // ternary is okay here, type enforced by QML
            color: conversationItem.focus ? focusColor : defaultColor
        }

    }
}
