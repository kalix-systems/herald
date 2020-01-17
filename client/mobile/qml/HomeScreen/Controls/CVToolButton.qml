import QtQuick.Controls 2.12
import QtQuick 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

// TODO move this into common & rename, it's an abstract animated icon button thing
ToolButton {
    property var onClicked : {
        throw "undefined callback"
    }
    property string imageSource: ""
    property color color: QmlCfg.palette.iconMatte
    property int iconSize: CmnCfg.iconSize

    background: Rectangle {
        id: splash
        color: QmlCfg.palette.iconMatte
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

    icon.source: imageSource
    icon.color: color
    icon.width: iconSize
    icon.height: iconSize

    ParallelAnimation {
        id: tapAnim
        NumberAnimation {
            target: splash
            property: "width"
            from: parent.height
            to: parent.height * 1.5
            duration: QmlCfg.units.shortDuration
            easing.type: Easing.InQuad
        }
        NumberAnimation {
            target: splash
            property: "opacity"
            from: 0.5
            to: 0
            duration: QmlCfg.units.shortDuration
            easing.type: Easing.InQuad
        }
        onFinished: {
            tapCallback()
        }
    }
}
