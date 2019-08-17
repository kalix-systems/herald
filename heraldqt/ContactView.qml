import QtQuick 2.13
import LibHerald 1.0

/// --- displays a list of contacts
ListView {

        spacing : 10
        clip: true
        delegate: Item {
            width: 80
            height: 40

            Row {
            ///--- Circle with initial
                Rectangle {
                    width: 40
                    height: 40
                    color: "#080909"
                    radius: 100
                    ///---- initial
                    Text {
                        text: qsTr(name[0])
                        color: "#FFFFFF"
                        anchors.centerIn: parent
                        bottomPadding: 1
                        font.pixelSize: parent.height - 5
                    }
                }

                Text {
                    text: name
                    font.bold: true
                    anchors.verticalCenter: parent.verticalCenter
                }

                spacing: 10
            }

        }

    }

