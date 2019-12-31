import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"

RowLayout {
    anchors {
        fill: parent
        rightMargin: CmnCfg.defaultMargin
        leftMargin: CmnCfg.defaultMargin
    }

    AnimIconButton {
        id: drawerButton
        Layout.alignment: Qt.AlignLeft
        color: CmnCfg.palette.iconFill
        imageSource: "qrc:/back-arrow-icon.svg"
        onClicked : {
            mainView.pop()
        }
    }

    Label {
        text: qsTr("Settings")
        Layout.alignment: Qt.AlignCenter
        Layout.fillWidth: true
        font: CmnCfg.headerFont
        color: CmnCfg.palette.iconFill
    }
}
