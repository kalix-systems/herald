import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../"

Column {
    width: parent.width

    id: wrapper
    spacing: CmnCfg.smallMargin
    Row {
        leftPadding: CmnCfg.defaultMargin
        height: notifLabel.height
        width: parent.width
        Label {
            id: notifLabel
            text: qsTr("Notifications")
            color: "black"
            wrapMode: Label.WrapAtWordBoundaryOrAnywhere
            font.pixelSize: CmnCfg.defaultFontSize
            font.family: CmnCfg.chatFont.name
            anchors.verticalCenter: parent.verticalCenter
        }
        Item {
            height: 10
            width: wrapper.width * 0.66 - notifLabel.width
        }

        ConfSwitch {
            checked: true
            anchors.verticalCenter: parent.verticalCenter
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }

    Row {
        leftPadding: CmnCfg.defaultMargin
        height: silentLabel.height
        width: parent.width
        Label {
            id: silentLabel
            text: qsTr("Silent")
            color: "black"
            wrapMode: Label.WrapAtWordBoundaryOrAnywhere
            font.pixelSize: CmnCfg.defaultFontSize
            font.family: CmnCfg.chatFont.name
            anchors.verticalCenter: parent.verticalCenter
        }

        Item {
            height: 10
            width: wrapper.width * 0.66 - silentLabel.width
        }
        ConfSwitch {
            checked: false
            anchors.verticalCenter: parent.verticalCenter
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }

    Row {
        leftPadding: CmnCfg.defaultMargin
        height: authorLabel.height
        width: parent.width
        Label {
            id: authorLabel
            text: qsTr("Show author name")
            color: "black"
            wrapMode: Label.WrapAtWordBoundaryOrAnywhere
            font.pixelSize: CmnCfg.defaultFontSize
            font.family: CmnCfg.chatFont.name
            anchors.verticalCenter: parent.verticalCenter
        }

        Item {
            height: 10
            width: wrapper.width * 0.66 - authorLabel.width
        }
        ConfSwitch {
            checked: true
            anchors.verticalCenter: parent.verticalCenter
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }
    Row {
        leftPadding: CmnCfg.defaultMargin
        height: messageLabel.height
        width: parent.width
        Label {
            id: messageLabel
            text: qsTr("Show message body")
            color: "black"
            wrapMode: Label.WrapAtWordBoundaryOrAnywhere
            font.pixelSize: CmnCfg.defaultFontSize
            font.family: CmnCfg.chatFont.name
            anchors.verticalCenter: parent.verticalCenter
        }
        Item {
            height: 10
            width: wrapper.width * 0.66 - messageLabel.width
        }

        ConfSwitch {
            checked: true
            anchors.verticalCenter: parent.verticalCenter
        }
    }
    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }
}
