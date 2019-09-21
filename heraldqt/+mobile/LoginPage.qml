import QtQuick 2.0
import QtQuick.Controls 2.12
import QtGraphicalEffects 1.13
import LibHerald 1.0

Rectangle {
    id: loginPage

    readonly property int heightUnit: root.height / 3
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
                source: "icons/land.png"
                fillMode: Image.PreserveAspectCrop
                layer.enabled: true
                layer.effect: OpacityMask {
                    maskSource: rect
                }
            }
        }
        /// username entry field
        TextArea {
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

            Keys.onReturnPressed: {
                heraldState.setConfigId(entryField.text.trim())
            }
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
            onClicked: {
                // TODO check length of id
                heraldState.setConfigId(entryField.text.trim())
            }
        }
    }

    Rectangle {
        color: Qt.darker("lightblue", 1.9)
        height: 60
        radius: QmlCfg.radius

        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
            rightMargin: root.width / 8
            leftMargin: root.width / 8
            verticalCenter: root.verticalCenter
            verticalCenterOffset: root.height / 4
        }


        MouseArea {
            anchors.fill: parent
        }

        Text {
            anchors.centerIn: parent
            color: "white"
            text: "Register Device To Existing Account ▸"
        }
    }
}
