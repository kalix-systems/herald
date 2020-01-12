import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../"

GridLayout {
    columns: 2
    rows: 8
    width: parent.width

    StandardLabel {
        text: qsTr("Notifications Enabled")
        color: "black"
        Layout.leftMargin: CmnCfg.defaultMargin
        wrapMode: Label.WrapAtWordBoundaryOrAnywhere
        font.pixelSize: CmnCfg.entityLabelSize
        font.family: CmnCfg.chatFont.name
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
        wrapMode: Label.WrapAtWordBoundaryOrAnywhere
        font.pixelSize: CmnCfg.entityLabelSize
        font.family: CmnCfg.chatFont.name
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
        wrapMode: Label.WrapAtWordBoundaryOrAnywhere
        font.pixelSize: CmnCfg.entityLabelSize
        font.family: CmnCfg.chatFont.name
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
        Layout.minimumWidth: 0
        Layout.leftMargin: CmnCfg.defaultMargin
        font.pixelSize: CmnCfg.entityLabelSize
        font.family: CmnCfg.labelFont.name
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
