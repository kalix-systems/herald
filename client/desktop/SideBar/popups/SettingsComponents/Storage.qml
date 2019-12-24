import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../../common" as CMN
import "qrc:/imports"

ColumnLayout {
    spacing: CmnCfg.defaultMargin
    RowLayout {
        Layout.fillWidth: true
        Layout.leftMargin: CmnCfg.defaultMargin
        Layout.topMargin: CmnCfg.defaultMargin
        Button {
            Layout.alignment: Qt.AlignCenter
            text: qsTr("Export Backup")
        }

        StandardLabel {
            text: qsTr("Last Backup Was : ") + "Never"
            color: "black"
            Layout.leftMargin: CmnCfg.defaultMargin
            font.pixelSize: CmnCfg.chatTextSize
        }
    }

    Button {
        Layout.leftMargin: CmnCfg.defaultMargin
        text: qsTr("Restore From Backup")
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
