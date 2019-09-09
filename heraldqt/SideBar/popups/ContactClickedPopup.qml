import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../../common" as Common
import "../../common/utils.mjs" as Utils

/// --- displays a list of contacts
Item {

    property alias optionsMenu: optionsMenu

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
            Keys.onReturnPressed: renameContact()
        }

        Button {
            text: "Submit"
            id: submissionButton
            anchors {
                bottom: parent.bottom
                right: parent.right
            }
            onClicked: renameContact()
        }
    }
}
