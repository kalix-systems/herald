import QtQuick 2.0
import QtQuick.Controls 2.12
import LibHerald 1.0

// TODO this is a FloatingActionButton, rename this and FABs.qml accordingly
Button {
    property string iconSource: ""
    property color iconColor: CmnCfg.palette.iconFill

    height: CmnCfg.fabDiameter
    width: height

    icon.source: iconSource
    icon.color: pressed ? Qt.lighter(iconColor, 1.3) : iconColor
    icon.height: CmnCfg.units.dp(24)
    icon.width: CmnCfg.units.dp(24)

    background: Rectangle {
        color: pressed ? Qt.darker(CmnCfg.palette.offBlack,
                                   1.3) : CmnCfg.palette.black
        anchors.fill: parent
        radius: height
    }
}
