import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "../../../common" as CMN
import "qrc:/imports" as Imports

GridLayout {
    columns: 2
    rows: 1
    columnSpacing: 0

    Imports.StandardLabel {
        text: qsTr("Theme")
        color: "black"
        font.pixelSize: CmnCfg.chatTextSize
        Layout.leftMargin: CmnCfg.defaultMargin
        Layout.fillWidth: true
    }

    Column {
        spacing: CmnCfg.microMargin
        Layout.rightMargin: CmnCfg.megaMargin

        ConfRadio {
            text: qsTr("Dark")
        }

        ConfRadio {
            text: qsTr("Light")
            checked: true
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
        Layout.columnSpan: 2
    }

}
