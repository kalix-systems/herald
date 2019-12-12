import QtQuick 2.0
import QtQuick.Controls 2.12
import QtGraphicalEffects 1.13
import LibHerald 1.0

Rectangle {
    readonly property int heightUnit: root.minimumHeight / 3
    id: registrationPage
    anchors.fill: parent

    LinearGradient {
        anchors.fill: parent
        gradient: Gradient {
            GradientStop {
                position: 0.0
                color: "lightblue"
            }
            GradientStop {
                position: 1.0
                color: Qt.darker("lightblue", 1.4)
            }
        }
    }

    Column {
        anchors.fill: parent
        anchors.margins: heightUnit
        spacing: 20

        /// --- crow emblem place holder
        Rectangle {
            id: rect
            anchors.horizontalCenter: parent.horizontalCenter
            width: heightUnit
            height: heightUnit
            radius: heightUnit

            Image {
                id: crow
                anchors.centerIn: rect
                height: rect.height
                width: rect.width
                source: "qrc:/land.png"
                fillMode: Image.PreserveAspectCrop
                layer.enabled: true
                layer.effect: OpacityMask {
                    maskSource: rect
                }
            }
        }

        /// username entry field
        TextField {
            id: entryField
            anchors.horizontalCenter: parent.horizontalCenter
            width: 150
            height: 25
            focus: true
            placeholderText: qsTr("Register Username...")
            background: Rectangle {
                color: "white"
                radius: 2
            }

            onTextChanged: entryField.text = text.trim()

            Keys.onReturnPressed: herald.registerNewUser(
                                      entryField.text.trim(),
                                      serverAddrTextField.text.trim(),
                                      serverPortTextField.text.trim())
        }

        TextField {
            id: serverAddrTextField
            anchors.horizontalCenter: parent.horizontalCenter
            width: 150
            height: 25
            placeholderText: qsTr("Server address")
        }

        TextField {
            id: serverPortTextField
            anchors.horizontalCenter: parent.horizontalCenter
            width: 150
            height: 25
            placeholderText: qsTr("Server port")
        }

        Button {
            anchors.horizontalCenter: parent.horizontalCenter
            width: 80
            height: 25
            Text {
                anchors.centerIn: parent
                color: "white"
                text: qsTr("Register")
            }
            background: Rectangle {
                color: "steelblue"
                radius: 3
            }
            onClicked: herald.registerNewUser(entryField.text.trim(),
                                              serverAddrTextField.text.trim(),
                                              serverPortTextField.text.trim())
        }
    }

    Rectangle {
        color: Qt.darker("lightblue", 1.9)
        height: 60

        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
        }

        Text {
            anchors.centerIn: parent
            color: "white"
            text: qsTr("Register Device To Existing Account ▸")
        }
    }
}
