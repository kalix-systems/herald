import QtQuick 2.4
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtGraphicalEffects 1.13
import LibHerald 1.0

ToolButton {
    property string source
    property color fill: CmnCfg.palette.iconMatte
    background: Item {}
    padding: 0
    icon.source: source
    icon.color: fill
    icon.width: 22
    icon.height: 22

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        onPressed: mouse.accepted = false
        cursorShape: Qt.PointingHandCursor
    }
}
