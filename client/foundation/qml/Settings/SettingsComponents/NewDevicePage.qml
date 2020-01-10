import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import LibHerald 1.0
import "../.."

Page {
    id: newDevicePage
    anchors.fill: parent
    ColumnLayout {
        anchors.fill: parent
        anchors.margins: CmnCfg.largeMargin * 4

        Text {
            Layout.alignment: Qt.AlignCenter
            Layout.preferredWidth: parent.width * 0.75
            text: qsTr("On your other device, if you have not registered, select `link to existing acount` from the landing page. Enter the words below into the text area presented to you on the page and press submit.
If you have registered a new account on your other device open `Settings` and navigate to `Add new Account` under devices,  press the button, enter the text show below into the text area, and press submit. ")
            wrapMode: Text.WrapAtWordBoundaryOrAnywhere
        }

        Label {
            id: text
            Layout.alignment: Qt.AlignCenter
            Layout.preferredWidth: parent.width * 0.66
            wrapMode: Text.WrapAtWordBoundaryOrAnywhere
            background: Rectangle {
                border.color: CmnCfg.palette.offBlack
                height: parent.height + CmnCfg.defaultMargin
                width: parent.width + CmnCfg.defaultMargin
                anchors.centerIn: parent
            }
            text: "" // some model should populate this with random text
        }

        RowLayout {
            Layout.alignment: Qt.AlignCenter
            TextButton {
                id: canButton
                text: qsTr("Cancel")
                onClicked: settingsFlickable.state = ""
            }
            TextButton {
                id: conButton
                text: qsTr("Connect")
                onClicked: {
                    parent.state = "waiting"
                    // call server API
                }
            }

            BusyIndicator {
                id: busy
                visible: false
            }

            states: [
                State {
                    name: "waiting"
                    PropertyChanges {
                        target: conButton
                        visible: false
                    }
                    PropertyChanges {
                        target: busy
                        visible: true
                    }
                }
            ]
        }
    }
}
