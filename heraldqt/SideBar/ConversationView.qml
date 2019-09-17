import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "../common/utils.mjs" as Utils
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
    visible: !gsContactsSearch

    clip: true
    currentIndex: -1
    boundsBehavior: Flickable.StopAtBounds

    Connections {
        target: appRoot
        onGsConversationIdChanged: {
            if (gsConversationId === undefined) {
                conversationList.currentIndex = -1
            }
        }
    }

    ScrollBar.vertical: ScrollBar {
    }

    delegate: Item {
        id: conversationItem
        //GS : rexporting the contact avatar to global state is a backwards ref!
        property Item conversationAvatar: conversationAvatar

        // This ternary is okay, types are enforced by QML
        height: visible ? 60 : 0
        width: parent.width

        Users {
            id: convoItemMembers
            conversationId: conversationsModel.conversationId(index)
        }

        /// NPB : This ought to be a mouse area with a hovered handler
        Rectangle {
            id: bgBox
            readonly property color focusColor: QmlCfg.palette.tertiaryColor
            readonly property color hoverColor: QmlCfg.palette.secondaryColor
            readonly property color defaultColor: QmlCfg.palette.mainColor

            Common.Divider {
                color: QmlCfg.palette.secondaryColor
                anchor: parent.bottom
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
                    conversationList.currentIndex = index
                    messageModel.conversationId = conversationId
                    appRoot.gsConversationId = conversationId
                    appRoot.gsConvoColor = QmlCfg.avatarColors[color]
                    appRoot.gsConvoItemMembers = convoItemMembers
                }

                // ternary is okay here, type enforced by QML
                onReleased: parent.state = containsMouse ? "hovering" : ""
            }
            // ternary is okay here, type enforced by QML
            color: conversationItem.focus ? focusColor : defaultColor
        }

        Common.Avatar {
            size: 50
            id: conversationAvatar
            displayName: Utils.unwrapOr(title, "unknown")
            colorHash: color
            pfpUrl: Utils.safeStringOrDefault(picture)
        }
    }
}
