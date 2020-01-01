import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"

RowLayout {
    anchors.fill: parent
    Row {
        Layout.alignment: Qt.AlignLeft
        Layout.leftMargin: CmnCfg.units.dp(12)
        spacing: CmnCfg.units.dp(16)
        AnimIconButton {
            id: backButton
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/back-arrow-icon.svg"
            onTapped: {
                mainView.pop(null)
            }
        }

        Label {
            id: stateLabel
            text: qsTr("New Group")
            font: CmnCfg.headerFont
            anchors.verticalCenter: parent.verticalCenter
            color: CmnCfg.palette.iconFill
        }
    }
}
