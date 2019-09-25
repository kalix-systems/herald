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

/// --- displays a list of contacts
ListView {
    id: contactList

    clip: true
    currentIndex: -1
    boundsBehavior: Flickable.StopAtBounds

    Connections {
        target: convModel
        onConversationIdChanged: {
            if (convModel.conversationId === undefined) {
                contactList.currentIndex = -1
            }
        }
    }

    ScrollBar.vertical: ScrollBar {
    }

    delegate: Item {
        id: contactItem
        //GS : rexporting the contact avatar to global state is a backwards ref!
        property Item contactAvatar: contactAvatar

        // This ternary is okay, types are enforced by QML
        height: visible ? 55 : 0
        width: parent.width
        visible: matched

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
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                onEntered: parent.state = "hovering"
                onExited: parent.state = ""

                onClicked: {
                    JS.contactClickHandler(mouse, contactList, index,
                                           pairwiseConversationId,
                                           popupManager.optionsMenu,
                                           convModel, appRoot)
                }

                // ternary is okay here, type enforced by QML
                onReleased: parent.state = containsMouse ? "hovering" : ""
            }

            Popups.ContactClickedPopup {
                id: popupManager
            }
            // ternary is okay here, type enforced by QML
            color: contactItem.focus ? focusColor : defaultColor
        }

        /// NPB: Make ALL calls to model proerties use the Explicit row syntax.
        /// NPB: unwrapOr should use a subset of falsey values to coerce to false, maybe make a tryGetOr(getter *fn , index, failValue)
        /// NB: Where is  index coming from?? (Positioner, but this is so implicit that we hate it)
        Common.Avatar {
            size: 45
            id: contactAvatar
            avatarLabel: displayName
            colorHash: color
            pfpUrl: Utils.safeStringOrDefault(profilePicture)
        }
    }
}
