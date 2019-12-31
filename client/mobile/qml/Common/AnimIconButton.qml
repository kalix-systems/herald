import QtQuick.Controls 2.12
import QtQuick 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

// TODO this should use IconButton in foundation
ToolButton {

    property string imageSource: ""
    property color color: CmnCfg.palette.iconMatte

    property size iconSize: Qt.size(CmnCfg.iconSize, CmnCfg.iconSize)

    background: Rectangle {
        id: splash
        color: CmnCfg.palette.iconMatte
        anchors.centerIn: parent
        opacity: 0
        height: width
        radius: height
    }

    TapHandler {
        onTapped: {
            tapAnim.running = true
        }
    }

    padding: 0
    icon.source: imageSource
    icon.color: color
    icon.width: iconSize.width
    icon.height: iconSize.height

    ParallelAnimation {
        id: tapAnim
        NumberAnimation {
            target: splash
            property: "width"
            from: parent.height
            to: parent.height * 1.5
            duration: CmnCfg.units.shortDuration
            easing.type: Easing.InQuad
        }
        NumberAnimation {
            target: splash
            property: "opacity"
            from: 0.5
            to: 0
            duration: CmnCfg.units.shortDuration
            easing.type: Easing.InQuad
        }
    }
}
