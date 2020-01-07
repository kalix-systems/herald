import QtQuick 2.4
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import LibHerald 1.0

ToolButton {
    id: button
    property string source
    // TODO rename this fillColor
    property color fill: CmnCfg.palette.iconMatte
    property alias mouseArea: mouse
    property alias tooltipText: tooltip.text
    background: Item {}
    padding: 0
    icon.source: source
    icon.color: fill
    icon.width: 22
    icon.height: 22
    MouseArea {
        id: mouse
        anchors.fill: parent
        hoverEnabled: true
        onPressed: mouse.accepted = false
        cursorShape: Qt.PointingHandCursor
        preventStealing: true
        z: parent.z + 1
    }

    ToolTip {
        id: tooltip
        y: -(height + CmnCfg.microMargin)
        visible: (mouseArea.containsMouse && tooltipText !== "")
        font.family: CmnCfg.chatFont.name
        font.pixelSize: 11
        padding: CmnCfg.microMargin
        delay: 1500
        timeout: 4000
        background: Rectangle {
            border.width: 1
            border.color: CmnCfg.palette.offBlack
        }
    }
}
