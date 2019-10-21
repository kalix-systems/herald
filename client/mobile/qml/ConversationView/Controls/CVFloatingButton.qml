import QtQuick 2.0
import QtQuick.Controls 2.12
import LibHerald 1.0

Button {
    property string iconSource: ""
    property color iconColor: QmlCfg.palette.iconMatte

    height: QmlCfg.units.dp(40)
    width: height

    icon.source: iconSource
    icon.color: pressed ? Qt.lighter(iconColor, 1.3) : iconColor
    icon.height: height
    icon.width: width

    background: Rectangle {
        color: pressed ? Qt.lighter(
                             QmlCfg.palette.tertiaryColor) : QmlCfg.palette.tertiaryColor
        anchors.fill: parent
        radius: height
        Image {}
    }
}
