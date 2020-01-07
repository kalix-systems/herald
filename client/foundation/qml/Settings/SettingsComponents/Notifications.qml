import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import '../../'

GridLayout {
    columns: 2
    rows: 8

    StandardLabel {
        text: qsTr("Notifications Enabled")
        color: "black"
        Layout.leftMargin: CmnCfg.defaultMargin
        font: CmnCfg.defaultFont
    }

    ConfSwitch {
        checked: true
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
        Layout.columnSpan: 2
    }

    StandardLabel {
        text: qsTr("Silent")
        color: "black"
        Layout.leftMargin: CmnCfg.defaultMargin
        font: CmnCfg.defaultFont
    }

    ConfSwitch {
        checked: false
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
        Layout.columnSpan: 2
    }

    StandardLabel {
        text: qsTr("Show author in notification")
        color: "black"
        Layout.leftMargin: CmnCfg.defaultMargin
        font: CmnCfg.defaultFont
    }

    ConfSwitch {
        checked: false
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
        Layout.columnSpan: 2
    }

    StandardLabel {
        text: qsTr("Show message body in notification")
        color: CmnCfg.palette.black
        Layout.leftMargin: CmnCfg.defaultMargin
        font: CmnCfg.defaultFont
    }

    ConfSwitch {
        checked: false
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
        Layout.columnSpan: 2
    }
}
