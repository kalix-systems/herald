import QtQuick 2.14
import LibHerald 1.0

// TODO: "TWO"
Item {
    anchors.fill: parent
    z: -1
    opacity: highlight || hoverHighlight ? 1.0 : 0.0
    Rectangle {
        anchors.fill: parent
        color: authorColor
        opacity: 0.2
    }
}
