import QtQuick 2.14
import LibHerald 1.0

Rectangle {
    anchors.fill: parent
    opacity: highlight || hoverHighlight ? 0.2 : 0.0
    z: -1
    color: authorColor
}
