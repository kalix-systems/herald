import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
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
                z: 10
                anchors.fill: parent
                onClicked: {
                    contactItem.focus = true
                    chatView.messageModel.conversationId = contact_id
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
