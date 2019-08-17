import QtQuick 2.12
import QtQuick.Controls 2.5
import QtQuick.Layouts 1.12
import LibHerald 1.0

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: qsTr("Contacts")
    header: ToolBar {
        Label {
            anchors.fill: parent
            text: qsTr("Contacts")
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
        }
    }

    Contacts {
        id: contacts
    }

    RowLayout {
        TextField {
            id: name_input
            placeholderText: "Add Contact"
            selectByMouse: true // this should be enabled on Desktop
            focus: true
            Keys.onReturnPressed: {
                contacts.add(name_input.text.trim())
                name_input.clear()
            }
        }
        Button {
            text: "add"
            onClicked: {
                contacts.add(name_input.text.trim())
                name_input.clear()
            }
        }
        Button {
            text: "Erase all contacts"
            onClicked: {
                contacts.remove_all()
            }
        }
    }

    ListView {
        id: contactsView
        anchors.topMargin: 65
        anchors.fill: parent
        model: contacts
        boundsBehavior: Flickable.StopAtBounds
        clip: true
        ScrollBar.vertical: ScrollBar {
            policy: ScrollBar.AlwaysOn
        }
        delegate: contactDelegate
    }

    Component {
        id: contactDelegate
        Item {
            width: 180
            height: 40
            Rectangle {
                anchors.fill: parent
                Text {
                    text: name
                }
            }
        }
    }
}
