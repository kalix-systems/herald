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
            text: qsTr("Language")
            color: CmnCfg.palette.black
            Layout.leftMargin: CmnCfg.defaultMargin
            font: CmnCfg.defaultFont
        }

        Item {
            Layout.fillWidth: true
        }

        StandardCombo {
            model: ["English", "Espańol", "Deutsche", "français", "et cetera"]
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }

    RowLayout {
        Layout.fillWidth: true
        Layout.rightMargin: CmnCfg.defaultMargin
        StandardLabel {
            text: qsTr("App Info")
            color: "black"
            Layout.leftMargin: CmnCfg.defaultMargin
            font: CmnCfg.defaultFont
        }

        Item {
            Layout.fillWidth: true
        }

        StandardLabel {
            text: qsTr("Version ") + "0.0.1-alpha.1-rc.1"
            color: "black"
            Layout.leftMargin: CmnCfg.defaultMargin
            font: CmnCfg.defaultFont
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
