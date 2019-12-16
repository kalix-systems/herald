import QtQuick 2.14
import LibHerald 1.0

//two rectangles to extend to both sides of pane
Item {
    anchors.fill: parent
    z: -1
    opacity: highlight == true ? 1.0 : 0.0
    Rectangle {
        width: convContainer.width
        anchors.right: parent.right
        color: CmnCfg.palette.medGrey
        anchors.verticalCenter: parent.verticalCenter
        height: parent.height + CmnCfg.smallMargin
    }

    Rectangle {
        width: convContainer.width
        anchors.left: parent.right
        color: CmnCfg.palette.medGrey
        anchors.verticalCenter: parent.verticalCenter
        height: parent.height + CmnCfg.smallMargin
    }
}
