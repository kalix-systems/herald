import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.12
import "../common"
/// --- displays a list of contacts
ListView {
    id: contactList
    boundsBehavior: Flickable.StopAtBounds
    clip: true
    currentIndex: -1
    ScrollBar.vertical: ScrollBar {
    }
    delegate: Item {
        property int rowHeight: 60

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
                    onTriggered: {
                        contacts.remove(index)
                        chatView.messageModel.clear_conversation_view()
                    }
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
                    return QmlCfg.palette.tertiaryColor
                } else {
                    return index % 2 ? QmlCfg.palette.secondaryColor : QmlCfg.palette.mainColor
                }
            }
        }
        ///TODO make and avatar component
        Avatar { displayName:  name ? name : contact_id
                 colorHash: color }
    }
}
