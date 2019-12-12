import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../../common" as CMN
import "qrc:/imports"

ColumnLayout {

    RowLayout {
        Layout.fillWidth: true
        StandardLabel {
            text: qsTr("Theme")
            color: "black"
            Layout.leftMargin: CmnCfg.margin
            font.pointSize: 14
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
