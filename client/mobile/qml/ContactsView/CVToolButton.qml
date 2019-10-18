import QtQuick.Controls 2.12
import QtQuick 2.12
import LibHerald 1.0

ToolButton {
    property var callback

    TapHandler {
        onTapped: {

        }
    }

    background: Image {
        id: icon
        property color color: QmlCfg.palette.iconMatte
    }

    Animation {
        id: cubicFade
        NumberAnimation {
            from: 0
            to: 200
            duration: 500
            easing.type: Easing.InOutQuad
        }
        NumberAnimation {
            from: 200
            to: 0
            duration: 500
            easing.type: Easing.InOutQuad
        }
    }
}
