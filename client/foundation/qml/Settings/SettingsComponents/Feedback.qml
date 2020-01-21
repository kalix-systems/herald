import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../"

ColumnLayout {
    spacing: CmnCfg.smallMargin

    StandardLabel {
        text: "<a href='https://github.com/kalix-systems/herald/wiki/Trouble-Shooting'>" + qsTr(
                  "Open help center") + "</a>"
        Layout.leftMargin: CmnCfg.defaultMargin
        Layout.topMargin: CmnCfg.smallMargin
        font.family: CmnCfg.chatFont.name
        font.pixelSize: CmnCfg.chatTextSize
        onLinkActivated: Qt.openUrlExternally(link)
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        Layout.fillWidth: true
    }

    StandardLabel {
        text: "<a href='https://github.com/kalix-systems/herald/issues'>" + qsTr(
                  "Report an issue") + "</a>"
        Layout.leftMargin: CmnCfg.defaultMargin
        font.family: CmnCfg.chatFont.name
        font.pixelSize: CmnCfg.chatTextSize
        onLinkActivated: Qt.openUrlExternally(link)
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        Layout.fillWidth: true
    }
}
