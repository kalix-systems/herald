import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import '../../'

ColumnLayout {
    spacing: CmnCfg.defaultMargin

    StandardLabel {
        text: qsTr("Open help center")
        color: "#0066cc"
        Layout.leftMargin: CmnCfg.defaultMargin
        Layout.topMargin: CmnCfg.smallMargin
        font: CmnCfg.defaultFont
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }

    StandardLabel {
        text: qsTr("Report an issue")
        color: "#0066cc"
        Layout.leftMargin: CmnCfg.defaultMargin
        font: CmnCfg.defaultFont
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
