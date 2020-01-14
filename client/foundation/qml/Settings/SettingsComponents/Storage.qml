import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../"

Column {
    id: wrapper
    width: parent.width
    spacing: CmnCfg.smallMargin

    Row {
        height: implicitHeight
        leftPadding: CmnCfg.defaultMargin
        GridLayout {
            anchors.verticalCenter: button.verticalCenter
            StandardLabel {
                id: save
                text: qsTr('Save conversation data to disk')
                color: CmnCfg.palette.black
                wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                font.family: CmnCfg.chatFont.name
                font.pixelSize: CmnCfg.chatTextSize
                Layout.maximumWidth: wrapper.width * 0.66 - CmnCfg.smallMargin
            }
        }
        Item {
            height: 10
            width: wrapper.width * 0.66 - save.width
        }

        TextButton {
            id: button
            text: qsTr("BACKUP")
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }

    Row {
        height: implicitHeight
        leftPadding: CmnCfg.defaultMargin
        GridLayout {
            anchors.verticalCenter: restorebutton.verticalCenter
            StandardLabel {
                id: load
                text: qsTr("Load backup")
                color: CmnCfg.palette.black
                wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                font.family: CmnCfg.chatFont.name
                font.pixelSize: CmnCfg.chatTextSize
                Layout.maximumWidth: wrapper.width * 0.66 - CmnCfg.smallMargin
            }
        }
        Item {
            height: 10
            width: wrapper.width * 0.66 - load.width
        }

        TextButton {
            id: restorebutton
            text: qsTr("RESTORE")
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }
}
