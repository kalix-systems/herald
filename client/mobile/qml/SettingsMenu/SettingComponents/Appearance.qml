import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "../../Common" as CMN
import "qrc:/imports" as Imports

ColumnLayout {

    RowLayout {
        Layout.fillWidth: true
        Imports.StandardLabel {
            text: qsTr("Theme")
            color: "black"
            Layout.leftMargin: CmnCfg.defaultMargin
            font.pixelSize: CmnCfg.chatTextSize
        }

        Item {
            Layout.fillWidth: true
        }

        ConfRadio {
            Layout.alignment: Qt.AlignRight
            text: qsTr("Dark")
        }

        ConfRadio {
            Layout.alignment: Qt.AlignRight
            text: qsTr("Light")
            checked: true
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
