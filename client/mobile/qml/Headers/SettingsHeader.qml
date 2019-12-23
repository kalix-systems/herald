import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"

RowLayout {
    anchors {
        fill: parent
        rightMargin: CmnCfg.margin
        leftMargin: CmnCfg.margin
    }

    IconButton {
        id: drawerButton
        Layout.alignment: Qt.AlignLeft
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/back-arrow-icon.svg"
        tapCallback: function () {
            mainView.pop()
        }
    }

    Label {
        text: qsTr("Settings")
        Layout.alignment: Qt.AlignCenter
        Layout.fillWidth: true
        font.pixelSize: CmnCfg.chatPreviewSize
        font.family: CmnCfg.chatFont.name
        color: CmnCfg.palette.iconFill
    }
}
