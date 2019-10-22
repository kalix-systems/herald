import QtQuick 2.0
import QtQuick.Controls 2.12
import LibHerald 1.0

Button {
    property string iconSource: ""
    property color iconColor: QmlCfg.palette.iconMatte

    height: QmlCfg.units.dp(56)
    width: height

    icon.source: iconSource
    icon.color: pressed ? Qt.lighter(iconColor, 1.3) : iconColor
    icon.height: QmlCfg.units.dp(24)
    icon.width: QmlCfg.units.dp(24)

    background: Rectangle {
        color: pressed ? Qt.lighter(QmlCfg.palette.secondaryColor,
                                    1.3) : QmlCfg.palette.secondaryColor
        anchors.fill: parent
        radius: height
    }
}
