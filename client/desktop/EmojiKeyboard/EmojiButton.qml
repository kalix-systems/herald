import QtQuick 2.13
import QtQuick.Controls 2.14
import LibHerald 1.0

Label {
    property bool takesModifier
    property string baseEmoji
    // consider this a private field
    property string resultEmoji: {
        if (takesModifier) {
            const swatchIndex = CmnCfg.skinSwatchList[CmnCfg.skinSwatchIndex]
            const firstSlice = baseEmoji.slice(0, 2)
            const secondSlice = baseEmoji.slice(2, baseEmoji.length)

            firstSlice + swatchIndex + secondSlice
        } else {
            baseEmoji
        }
    }
    property color lowlight: CmnCfg.palette.darkGrey
    height: selector.height + CmnCfg.smallMargin
    width: selector.width + CmnCfg.smallMargin

    MouseArea {
        id: hoverHandler
        anchors.fill: bg
        hoverEnabled: true
        propagateComposedEvents: true
        onClicked: maskShape.send(resultEmoji, takesModifier)
    }

    background: Rectangle {
        id: bg
        visible: parent.pressed || hoverHandler.containsMouse
        anchors.fill: parent
        anchors.centerIn: parent
        color: lowlight
    }

    Text {
        id: selector
        anchors.centerIn: parent
        color: "white"
        font.pixelSize: 15
        text: resultEmoji
    }
}
