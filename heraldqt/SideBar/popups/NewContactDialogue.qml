import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// RS: Rusts job
// Factor Component: FC

// NPB: this should maybe be a new
// window or popup that feels more robust.
// adding someone should feel kinda rewarding.
Popup {
    id: newContactDialogue
    modal: true
    focus: true
    closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
    width: 300
    height: 200

    // TS: also this should take args
    function insertContact() {
        if (entryField.text.trim().length === 0)
            return
        contactsModel.add(entryField.text.trim())
        entryField.clear()
        newContactDialogue.close()
    }

    TextArea {
        id: entryField
        focus: true
        placeholderText: qsTr("Enter contact name")
        Keys.onReturnPressed: insertContact()
    }

    Button {
        id: submissionButton
        text: "submit"
        anchors {
            bottom: parent.bottom
            right: parent.right
        }
        onClicked: insertContact()
    }
}
