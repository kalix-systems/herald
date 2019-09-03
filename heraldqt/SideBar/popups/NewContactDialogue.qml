import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12

Popup {

    id: newContactDialogue
    modal: true
    focus: true
    closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
    width: 300
    height: 200

    function insertContact() {
        if (entryField.text.trim().length === 0)
            return
        contacts.add(entryField.text.trim())
        entryField.clear()
        newContactDialogue.close()
    }

    TextArea {
        focus: true
        id: entryField
        placeholderText: qsTr("Enter contact name")
        Keys.onReturnPressed: insertContact()
    }

    Button {
        text: "submit"
        id: submissionButton
        anchors {
            bottom: parent.bottom
            right: parent.right
        }
        onClicked: insertContact()
    }
}
