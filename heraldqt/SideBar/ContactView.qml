import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "../common/utils.js" as Utils
import "popups" as Popups

/// --- displays a list of contacts
ListView {
    id: contactList
    boundsBehavior: Flickable.StopAtBounds
    clip: true
    currentIndex: -1

    ScrollBar.vertical: ScrollBar {
    }
    delegate: Item {
        id: contactItem
        height: {
            if (visible)
                60
        }

        width: parent.width
        visible: matched

        Rectangle {
            id: bgBox
            property color focusColor: QmlCfg.palette.tertiaryColor
            property color hoverColor: QmlCfg.palette.secondaryColor
            property color defaultColor: QmlCfg.palette.mainColor

            Rectangle {
                anchors.verticalCenter: parent.bottom
                color: QmlCfg.palette.secondaryColor
                width: parent.width
                height: 1.5
            }

            anchors.fill: parent

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
                // Note : this is really imperative, we should do this somehow else.
                onClicked: {
                    if (mouse.button === Qt.LeftButton) {
                        contactItem.focus = true
                        chatView.messageModel.conversationId = contact_id
                        chatView.messageBar.chatBarAvatar.displayName = contactAvatar.displayName
                        chatView.messageBar.chatBarAvatar.pfpUrl = contactAvatar.pfpUrl
                        chatView.messageBar.chatBarAvatar.colorHash = contactAvatar.colorHash
                        chatView.state = "visibleview"
                    } else {

                        popupManager.optionsMenu.x = mouse.x
                        popupManager.optionsMenu.y = mouse.y
                        popupManager.optionsMenu.open()
                    }
                }

                onReleased: {
                    if (containsMouse) {
                        parent.state = "hovering"
                    } else {
                        parent.state = ""
                    }
                }
            }

            Popups.ContactClickedPopup {
                id: popupManager
            }

            color: {
                if (contactItem.focus) {
                    return focusColor
                } else {
                    return defaultColor
                }
            }
        }

        Common.Avatar {
            size: 50
            id: contactAvatar
            displayName: Utils.unwrap_or(name, contact_id)
            colorHash: color
            pfpUrl: Utils.unwrap_or(profile_picture, null)
        }
    }
}
