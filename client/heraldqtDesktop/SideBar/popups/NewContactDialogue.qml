import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "./NewContactDialogue.mjs" as JS
import QtQuick.Window 2.13

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
Window {
    id: newContactDialogue
    width: 400
    height: 200
    maximumHeight: height
    minimumHeight: height
    maximumWidth: width
    minimumWidth: width

    Component.onCompleted: {
        x = root.x + root.width / 3
        y = root.y + 100
    }

    //eventually more stuff should be added here
    TextField {
        id: entryArea
        anchors.horizontalCenter: parent.horizontalCenter
        focus: true
        placeholderText: qsTr("Enter contact ID")
        Keys.onReturnPressed: JS.insertContact(newContactDialogue, entryArea,
                                               contactsModel, networkHandle, conversationsModel)
    }

    Button {
        id: submissionButton
        text: "Submit"
        anchors {
            bottom: parent.bottom
            right: parent.right
        }
       onClicked: JS.insertContact(newContactDialogue, entryArea,
                                    contactsModel, networkHandle, conversationsModel)
    }
}
