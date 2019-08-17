import QtQuick 2.13
import "PlaceHolderData.qml"

ListView {
        spacing : 10
        height : parent.height
        delegate: Item {
            x: 5
            width: 80
            height: 40
            Row {
                id: row1
                Rectangle {
                    width: 40
                    height: 40
                    color: colorCode
                }

                Text {
                    text: name
                    font.bold: true
                    anchors.verticalCenter: parent.verticalCenter
                }
                spacing: 10
            }

        }
        model: PlaceHolderData {}
    }

