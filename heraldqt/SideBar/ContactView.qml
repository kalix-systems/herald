import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.12

/// --- displays a list of contacts
ListView {
    id: contactList
    boundsBehavior: Flickable.StopAtBounds
    clip: true
    ScrollBar.vertical: ScrollBar {
    }

    delegate: Item {
        property int rowHeight: 60
        property string displayName: name ? name : contact_id
        id: contactItem
        height: rowHeight
        width: parent.width
        Rectangle {

            MouseArea {
                acceptedButtons: Qt.LeftButton | Qt.RightButton
                z: 10
                anchors.fill: parent
                onClicked: {
                    if (mouse.button === Qt.LeftButton) {
                        contactItem.focus = true
                        chatView.messageModel.conversationId = contact_id
                    } else {
                        optionsMenu.x = mouse.x
                        optionsMenu.y = mouse.y
                        optionsMenu.open()
                    }
                }
            }

            Menu {
                id: optionsMenu
                closePolicy: Popup.CloseOnPressOutside
                MenuItem {
                    text: 'Delete Contact'
                    onTriggered: contacts.remove(index)
                }
                MenuItem {
                    text: 'Rename Contact'
                    onTriggered: renameContactDialogue.open()
                }
            }

            function renameContact() {
                if (entryField.text.trim().length == 0) {
                    return
                }
                contactItem.displayName = entryField.text.trim()
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
            width: parent.width
            height: rowHeight
            color: {
                if (contactItem.focus) {
                    return "lightsteelblue"
                } else {
                    return index % 2 ? "#f3f3f3" : "white"
                }
            }
        }

        Row {
            ///--- Circle with initial
            leftPadding: 10
            anchors.verticalCenter: parent.verticalCenter
            Rectangle {
                width: rowHeight - 10
                height: rowHeight - 10
                anchors.verticalCenter: parent.verticalCenter
                color: "#000000"
                radius: 100
                ///---- initial
                Text {
                    text: qsTr(displayName[0].toUpperCase())
                    font.bold: true
                    color: "#FFFFFF"
                    anchors.centerIn: parent
                    font.pixelSize: parent.height - 5
                }
            }

            Text {
                text: displayName
                font.bold: true
                anchors.verticalCenter: parent.verticalCenter
            }

            spacing: 10
        }
    }
}
