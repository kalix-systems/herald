import QtQuick 2.14
import LibHerald 1.0

//two rectangles to extend to both sides of pane
Item {
    anchors.fill: parent
    z: -1
    opacity: highlight || hoverHighlight ? 1.0 : 0.0
    Rectangle {
        anchors.fill: parent
        color: authorColor
        opacity: 0.15
    }
}
