import QtQuick.Controls 2.12
import QtQuick 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

// TODO this should use IconButton in foundation
ToolButton {
    id: tb
    property string imageSource: ""
    property color color: CmnCfg.palette.iconMatte
    property size iconSize: Qt.size(CmnCfg.iconSize, CmnCfg.iconSize)
    signal tapped

    TapHandler {
        onTapped: {
            tapAnim.start()
            parent.tapped()
        }
    }

    // buttons "onClicked" property does not work on mobile
    // so I made a tapped signal, because there is not one by defaultx
    background: Rectangle {
        id: splash
        anchors.centerIn: parent
        color: CmnCfg.palette.iconMatte
        opacity: 0
        height: tb.height / 2
        width: height
        radius: height
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
            property: "height"
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
