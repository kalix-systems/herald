import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "../common/utils.mjs" as Utils
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

    ScrollBar.vertical: ScrollBar {}

    delegate: Item {
        id: contactItem

        //GS : rexporting the contact avatar to global state is a backwards ref!
        property Item contactAvatar: contactAvatar

        // TS: yes we have a bunch of stupid functions that do one thing
        height: if (visible)  60
        width: parent.width
        visible: matched

        /// NPB : THis ought to be a mouse area with a hovered handler
        Rectangle {
            id: bgBox
            readonly property color focusColor: QmlCfg.palette.tertiaryColor
            readonly property color hoverColor: QmlCfg.palette.secondaryColor
            readonly property color defaultColor: QmlCfg.palette.mainColor

            /// FC: ANOTHER BORDER!
            Rectangle {
                anchors.verticalCenter: parent.bottom
                color: QmlCfg.palette.secondaryColor
                width: parent.width
                height: 1.5
            }

            anchors.fill: parent

            /// Note: can we use the highlight property here
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
                onEntered: {
                    parent.state = "hovering"
                }
                onExited: {
                    parent.state = ""
                }
                //TS: This should pass ... some event objects and other context specific goodies.
                onClicked: {
                    if (mouse.button === Qt.LeftButton) {
                        currentIndex = index
                        print(currentIndex, currentItem.contactAvatar.displayName)
                        contactItem.focus = true
                        messageModel.conversationId = contact_id
                        chatView.state = "visibleview"
                    } else {
                        // NPB: this should not *really* be a popup, I wish we had a native widgets solution.
                        // import Qt.labs.platform 1.1 !??!?!?
                        popupManager.optionsMenu.x = mouse.x
                        popupManager.optionsMenu.y = mouse.y
                        popupManager.optionsMenu.open()
                    }
                }

                // TS?: maybe make a "safeSwitch" function so this one line and in TS
                onReleased: {
                    if (containsMouse) {
                        parent.state = "hovering"
                    } else {
                        parent.state = ""
                    }
                }
            }

            ///NPB : see the QT labs menu import. [https://doc.qt.io/qt-5/qml-qt-labs-platform-menu.html]
            Popups.ContactClickedPopup {
                id: popupManager
            }
            // TS?: maybe make a "safeSwitch" function so this one line and in TS
            color: {
                if (contactItem.focus) {
                    return focusColor
                } else {
                    return defaultColor
                }
            }
        }

        /// NPB: Make ALL calls to model proerties use the Explicit row syntax.
        /// NPB: unwrapOr should use a subset of falsey values to coerce to false, maybe make a tryGetOr(getter *fn , index, failValue)
        /// NB: Where is  index coming from?? (Positioner, but this is so implicit that we hate it)
        Common.Avatar {
            size: 50
            id: contactAvatar
            /// NPB: use camel case in libherald please
            displayName: Utils.unwrapOr(name, contact_id)
            colorHash: color
            /// NPB: use camel case in libherald please
            pfpUrl: Utils.unwrapOr(profile_picture, null)
        }
    }
}
