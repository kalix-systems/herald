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
        anchors.margins: CmnCfg.largeMargin * 2

        Label {
            Layout.alignment: Qt.AlignCenter
            text: qsTr("Title")
        }

        Text {
            Layout.alignment: Qt.AlignCenter
            Layout.preferredWidth: parent.width * 0.66
            text: qsTr("instructions instructions instructions instructions instructions instructions \
instructions instructions instructions instructions instructions instructions \
iinstructions instructions  instructions instructions")
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

            text: "boogaloo nine tryhard lemon sasquatch dogwater"
        }

        RowLayout {
            Layout.alignment: Qt.AlignCenter
            TextButton {
                text: qsTr("Cancel")
                onClicked: settingsFlickable.state = ""
            }
            TextButton {
                text: qsTr("Confirm")
            }
        }
    }
}
