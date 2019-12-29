import QtQuick.Controls 2.12
import QtQuick 2.12
import LibHerald 1.0

// Text field with standard 1px bottom border
TextField {
    id: textField
    property alias borderColor: border.color

    leftPadding: 0
    rightPadding: 0
    bottomPadding: CmnCfg.units.dp(2)
    placeholderText: "Enter text"
    font: CmnCfg.defaultFont
    color: CmnCfg.palette.white

    background: Rectangle {
        id: border
        height: 1
        color: CmnCfg.palette.white
        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
        }
    }
}
