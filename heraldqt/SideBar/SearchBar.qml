import QtQuick 2.0
import QtQuick.Controls 2.13
import LibHerald 1.0

ToolBar {
    id: searchBar
    anchors.left: parent.left
    y: toolBar.y + toolBar.height
    width: contactPane.width
    height: 40
    font.pointSize: 25
    background: Rectangle {
        anchors.fill: parent
        color: "#FFFFFF"
    }

    ///--- Add contact button
    Button {
        id: addContactButton
        font.pointSize: parent.height - 10
        height: parent.height - 15
        anchors.rightMargin: 10
        anchors.verticalCenterOffset: 0
        anchors.right: parent.right
        anchors.verticalCenter: parent.verticalCenter
        width: height

        background: Rectangle {
            id: bg
            color: "#3c7c9b"
            radius: 100
            Image {
                source: "qrc:///icons/plus.png"
                anchors.fill: parent
                scale: 0.7
            }
        }

        MouseArea {
            anchors.fill: parent
            hoverEnabled: true
            onEntered: {
                bg.color = Qt.darker(bg.color, 1.5)
            }
            onExited: {
                bg.color = Qt.lighter(bg.color, 1.5)
            }
            onPressed: {
                bg.color = Qt.darker(bg.color, 2.5)
            }
            onReleased: {
                bg.color = Qt.lighter(bg.color, 2.5)
            }
            onClicked: {
                newContactDialogue.open()
            }
        }
    }

   function insertContact() {
                    if (entryField.text.trim().length === 0) {
                        return
                    }
                    contacts.add(entryField.text.trim())
                    entryField.clear()
                    newContactDialogue.close()
                }

    Popup {
        id: newContactDialogue
        modal: true
        focus: true
        closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
        width: 300
        height: 200
        anchors.centerIn: root

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


}


