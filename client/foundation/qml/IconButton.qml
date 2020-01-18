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
    icon.width: CmnCfg.iconSize
    icon.height: CmnCfg.iconSize

    MouseArea {
        id: mouse
        anchors.fill: parent
        hoverEnabled: true
        acceptedButtons: Qt.NoButton
        cursorShape: Qt.PointingHandCursor
        preventStealing: true
        z: parent.z + 1
    }

    ToolTip {
        id: tooltip
        y: -(height + CmnCfg.microMargin)
        visible: (mouseArea.containsMouse && tooltipText !== "")
        font.pixelSize: 11

        contentItem: Text {
            text: tooltip.text
            font: tooltip.font
            color: CmnCfg.sysPalette.text
        }

        padding: CmnCfg.microMargin
        delay: 1500
        timeout: 4000
        background: Rectangle {
            color: CmnCfg.sysPalette.window
            border.width: 1
            border.color: CmnCfg.sysPalette.midlight
        }
    }
}
