import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import Qt.labs.platform 1.0
import "../../"

Column {
    id: wrapper

    spacing: CmnCfg.smallMargin
    width: parent.width
    Row {
        height: implicitHeight
        leftPadding: CmnCfg.defaultMargin

        GridLayout {
            anchors.verticalCenter: deviceButton.verticalCenter
            StandardLabel {
                id: deviceLabel
                text: qsTr("New devices")
                color: "black"
                font.family: CmnCfg.chatFont.name
                font.pixelSize: CmnCfg.chatTextSize
            }
        }

        Item {
            height: 10
            width: wrapper.width * 0.66 - deviceLabel.width
        }

        TextButton {
            id: deviceButton
            text: qsTr("LINK")
            onClicked: settingsFlickable.state = "newDeviceFlow"
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: wrapper.width
    }
    StandardLabel {
        text: qsTr("Current devices")

        color: CmnCfg.palette.black
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.defaultMargin
        font.family: CmnCfg.chatFont.name
        font.pixelSize: CmnCfg.chatTextSize
    }

    //    Column {
    //        Layout.fillWidth: true
    //        Repeater {
    //            model: 0
    //            width: parent.width
    //            Item {
    //                id: deviceInformation
    //                MouseArea {
    //                    anchors.fill: parent
    //                    onClicked: {

    //                        // show options for invalidating other devices
    //                    }
    //                }
    //            }
    //        }
    //    }
    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }
}
