import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.12
import QtQuick.Dialogs 1.3
import "../common" as Common

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
            anchors.fill: parent
            MouseArea {
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                z: 10
                anchors.fill: parent
                onClicked: {
                    if (mouse.button === Qt.LeftButton) {
                        contactItem.focus = true
                        chatView.messageModel.conversationId = contact_id
                        chatView.messageBar.chatBarAvatar.displayName = contactAvatar.displayName
                        chatView.messageBar.chatBarAvatar.pfpUrl = contactAvatar.pfpUrl
                        chatView.messageBar.chatBarAvatar.colorHash = contactAvatar.colorHash
                    } else {
                        optionsMenu.x = mouse.x
                        optionsMenu.y = mouse.y
                        optionsMenu.open()
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
                        contactAvatar.pfpUrl = null
                        chatView.messageBar.chatBarAvatar.pfpUrl = null
                        contacts.setProfile_picture(index, "")
                        //TODO: delete profile picture from database function
                    }
                }
            }

            function renameContact() {
                if (entryField.text.trim().length == 0) {
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

            id: bgBox
            color: {
                if (contactItem.focus) {
                    return QmlCfg.palette.tertiaryColor
                } else {
                    return index % 2 ? QmlCfg.palette.secondaryColor : QmlCfg.palette.mainColor
                }
            }
        }

        Common.Avatar {
            size: 50
            id: contactAvatar
            displayName: name ? name : contact_id
            colorHash: color
            pfpUrl: profile_picture === undefined ? "" : profile_picture
        }
    }
}
