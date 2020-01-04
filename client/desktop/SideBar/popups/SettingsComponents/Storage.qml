import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../../common" as CMN
import "qrc:/imports"

GridLayout {
    rows: 2
    columns: 2

    StandardLabel {
        text: qsTr('Save conversation data to this device')
        font: CmnCfg.defaultFont
        color: CmnCfg.palette.black
        Layout.fillWidth: true
        Layout.leftMargin: CmnCfg.defaultMargin
    }

    TextButton {
        text: qsTr("BACK UP")
        Layout.alignment: Qt.AlignCenter
        Layout.rightMargin: CmnCfg.megaMargin
    }

    StandardLabel {
        text: qsTr("Last Backup Was : ") + "Never"
        color: CmnCfg.palette.black
        font: CmnCfg.defaultFont
        Layout.fillWidth: true
        Layout.leftMargin: CmnCfg.defaultMargin
    }

    TextButton {
        text: qsTr("RESTORE")
        Layout.alignment: Qt.AlignCenter
        Layout.rightMargin: CmnCfg.megaMargin
    }

}
