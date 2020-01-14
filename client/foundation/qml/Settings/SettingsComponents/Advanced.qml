import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import LibHerald 1.0
import "../../"

ColumnLayout {

    RowLayout {
        Layout.fillWidth: true
        StandardLabel {
            text: qsTr("Language")
            color: CmnCfg.palette.black
            Layout.leftMargin: CmnCfg.defaultMargin
            Layout.fillWidth: true
            font: CmnCfg.defaultFont
        }

        StandardCombo {
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: CmnCfg.largeMargin
            model: ["English"]
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        Layout.fillWidth: true
    }

    RowLayout {
        Layout.fillWidth: true
        Layout.rightMargin: CmnCfg.defaultMargin
        StandardLabel {
            text: qsTr("App info")
            color: "black"
            Layout.leftMargin: CmnCfg.defaultMargin
            font: CmnCfg.defaultFont
        }

        Item {
            Layout.fillWidth: true
        }

        StandardLabel {
            text: qsTr("version ") + "0.0.1-alpha"
            color: CmnCfg.palette.offBlack
            Layout.leftMargin: CmnCfg.defaultMargin
            font: CmnCfg.defaultFont
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        Layout.fillWidth: true
    }
}
