import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../../common" as CMN
import "qrc:/imports"

ColumnLayout {
    spacing: CmnCfg.margin
    RowLayout {
        Layout.fillWidth: true
        Layout.leftMargin: CmnCfg.margin
        Layout.topMargin: CmnCfg.margin
        Button {
            Layout.alignment: Qt.AlignCenter
            text: qsTr("Export Backup")
        }

        StandardLabel {
            text: qsTr("Last Backup Was : ") + "Never"
            color: "black"
            Layout.leftMargin: CmnCfg.margin
            font.pointSize: 14
        }
    }

    Button {
        Layout.leftMargin: CmnCfg.margin
        text: qsTr("Restore To Backup")
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
