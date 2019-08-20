import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12

/// --- displays a list of contacts
ListView {
    Layout.leftMargin: 0
    boundsBehavior: Flickable.StopAtBounds
    clip: true

    delegate: Item {
        property int rowHeight: 60
        property string displayName: name ? name : contact_id
        height: rowHeight
        width: parent.width

        Rectangle {
            id: bgBox
            width: parent.width
            height: rowHeight
            color: index % 2 ? "#f3f3f3" : "white"
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
