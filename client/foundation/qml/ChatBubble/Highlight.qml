import QtQuick 2.14
import LibHerald 1.0

//two rectangles to extend to both sides of pane
Item {
    anchors.fill: parent
    z: -1
    opacity: highlight == true ? 1.0 : 0.0
    Rectangle {
        anchors.fill: parent
        color: CmnCfg.palette.medGrey
    }
}
