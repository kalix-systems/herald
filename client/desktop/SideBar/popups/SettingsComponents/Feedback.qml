import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../../common" as CMN
import "qrc:/imports"

ColumnLayout {
    spacing: CmnCfg.defaultMargin

    Item {
        height: CmnCfg.defaultMargin
        width: 1
    }

    StandardLabel {
        text: qsTr("Open help center")
        color: "#0066cc"
        Layout.leftMargin: CmnCfg.defaultMargin
        font.pixelSize: CmnCfg.chatTextSize
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
        font.pixelSize: CmnCfg.chatTextSize
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
