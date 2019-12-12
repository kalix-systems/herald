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
            text: qsTr("Default message exipration time")
            color: "black"
            Layout.leftMargin: CmnCfg.margin
            font.pointSize: 14
        }

        Item {
            Layout.fillWidth: true
        }

        StandardCombo {
            model: ["Off", "1 Minute", "1 Hour", "1 Day", "1 Week", "1 Month", "1 Year"]
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
