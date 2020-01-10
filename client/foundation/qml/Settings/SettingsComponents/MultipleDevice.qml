import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import Qt.labs.platform 1.0
import "../../"

ColumnLayout {

    TextButton {
        Layout.leftMargin: CmnCfg.defaultMargin
        text: "Link New Device"
        onClicked: settingsFlickable.state = "newDeviceFlow"
    }

    StandardLabel {
        text: qsTr("Current Devices")
        color: CmnCfg.palette.black
        Layout.leftMargin: CmnCfg.defaultMargin
        Layout.fillWidth: true
        font: CmnCfg.defaultFont
    }

    Column {
        Layout.fillWidth: true
        Repeater {
            model: 0
            width: parent.width
            Item {
                id: deviceInformation
                MouseArea {
                    anchors.fill: parent
                    onClicked: {

                        // show options for invalidating other devices
                    }
                }
            }
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
