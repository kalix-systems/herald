import QtQuick 2.4
import QtQuick.Controls 2.13

Rectangle {
    property var bottomAnchor
    property var horizontal: false
    property var leftAnchor
    anchors.bottom: horizontal ? undefined : bottomAnchor
    anchors.left: horizontal ? leftAnchor : undefined
    width: horizontal ? 1 : parent.width
    height: horizontal ? parent.width : 1
}
