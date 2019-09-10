import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "./NewContactDialogue.mjs" as JS

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

    TextArea {
        id: entryArea
        focus: true
        placeholderText: qsTr("Enter contact name")
        Keys.onReturnPressed: JS.insertContact(newContactDialogue, entryArea,
                                               contactsModel)
    }

    Button {
        id: submissionButton
        text: "submit"
        anchors {
            bottom: parent.bottom
            right: parent.right
        }
        onClicked: JS.insertContact(newContactDialogue, entryArea,
                                    contactsModel)
    }
}
