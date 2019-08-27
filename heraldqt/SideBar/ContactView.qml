import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.12
import QtQuick.Dialogs 1.3
import "../common" as Common
import "../common/utils.js" as Utils

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
        height: 60
        width: parent.width

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
                onClicked: {
                    if (mouse.button === Qt.LeftButton) {
                        contactItem.focus = true
                        chatView.messageModel.conversationId = contact_id
                        chatView.messageBar.chatBarAvatar.displayName = contactAvatar.displayName
                        chatView.messageBar.chatBarAvatar.pfpUrl = contactAvatar.pfpUrl
                        chatView.messageBar.chatBarAvatar.colorHash = contactAvatar.colorHash
                        chatView.state = "visibleview"
                    } else {
                        optionsMenu.x = mouse.x
                        optionsMenu.y = mouse.y
                        optionsMenu.open()
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

            FileDialog {
                id: pfpDialog
                onSelectionAccepted: {
                    var retCode = contacts.setProfile_picture(index, fileUrl)
                    if (retCode) {
                        contactAvatar.pfpUrl = profile_picture
                        chatView.messageBar.chatBarAvatar.pfpUrl = profile_picture
                    } else
                        print("TODO: Error popup here...")
                    close()
                }
            }

            Menu {

                id: optionsMenu
                closePolicy: Popup.CloseOnPressOutside
                MenuItem {
                    text: 'Delete Contact'
                    onTriggered: {
                        if (contact_id === chatView.messageModel.conversationId)
                            chatView.state = "" //TODO clearview should be less imperative
                        contacts.remove(index)
                        chatView.messageModel.clear_conversation_view()
                    }
                }
                MenuItem {
                    text: 'Rename Contact'
                    onTriggered: renameContactDialogue.open()
                }

                MenuItem {
                    text: 'Choose Avatar'
                    onTriggered: pfpDialog.open()
                }

                MenuItem {
                    text: 'Clear Avatar'
                    onTriggered: {
                        contactAvatar.pfpUrl = ""
                        chatView.messageBar.chatBarAvatar.pfpUrl = ""
                        contacts.setProfile_picture(index, "")
                    }
                }
            }

            function renameContact() {
                if (entryField.text.trim() === "") {
                    return
                }
                name = entryField.text.trim()
                print(contact_id, chatView.messageBar.contact_id)
                if (contact_id === chatView.messageModel.conversationId) {
                    chatView.messageBar.chatBarAvatar.displayName = name
                }
                entryField.clear()
                renameContactDialogue.close()
            }

            Popup {
                id: renameContactDialogue
                closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
                width: 300
                height: 100

                TextArea {
                    focus: true
                    id: entryField
                    placeholderText: qsTr("Enter new name")
                    Keys.onReturnPressed: bgBox.renameContact()
                }

                Button {
                    text: "Submit"
                    id: submissionButton
                    anchors {
                        bottom: parent.bottom
                        right: parent.right
                    }
                    onClicked: bgBox.renameContact()
                }
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
