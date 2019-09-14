import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "../../common" as Common
import "../../common/utils.mjs" as Utils
import "./ContactClickedPopup.mjs" as JS

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// RS: Rusts job
// Factor Component: FC

/// --- displays a list of contacts
Item {
    property alias optionsMenu: optionsMenu
    // GS: These should be bound to global scope. handled ENTIRELY
    // by the Contacts model functions.
    FileDialog {
        id: pfpDialog
        nameFilters: ["(*.jpg *.png *.jpeg)"]
        folder: shortcuts.desktop
        onSelectionAccepted: {
            // TS:
            var retCode = contactsModel.setProfilePicture(index, fileUrl)
            if (retCode) {
                contactAvatar.pfpUrl = profilePicture
            } else
                print("TODO: Native Error popup here...")
            close()
        }
    }

    Menu {
        id: optionsMenu

        MenuItem {
            text: 'Delete Contact'
            //TS: this should be in typescript
            onTriggered: JS.deleteContact(pairwiseConversationId, index,
                                          contactsModel, messageModel, appRoot)
        }

        MenuSeparator {
        }

        MenuItem {
            text: 'Rename Contact'
            // Note: remove , because this is a testing feature.
            // instead networking needs to know...
            onTriggered: renameContactDialogue.open()
        }

        MenuSeparator {
        }

        MenuItem {
            // Note: remove , because this is a testing feature
            // instead networking needs to know...
            text: 'Choose Avatar'
            onTriggered: pfpDialog.open()
        }

        MenuItem {
            // Note: remove , because this is a testing feature.
            // instead networking needs to know...
            text: 'Clear Avatar'
            onTriggered: {
                contactAvatar.pfpUrl = ""
                contactsModel.setProfilePicture(index, "")
            }
        }
    }

    // TS: but also try to make disallowed keys work
    function renameContact() {
        if (entryField.text.trim() === "") {
            return
        }
        name = entryField.text.trim()
        entryField.clear()
        renameContactDialogue.close()
    }

    Popup {
        id: renameContactDialogue
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
        width: 300
        height: 100

        TextArea {
            id: entryField
            focus: true
            placeholderText: qsTr("Enter new name")
            Keys.onReturnPressed: renameContact()
            anchors.fill: parent
            wrapMode: TextEdit.WrapAnywhere
        }

        Button {
            id: submissionButton
            text: "Submit"
            anchors {
                bottom: parent.bottom
                right: parent.right
            }
            //again TS:
            onClicked: renameContact()
        }
    }
}
