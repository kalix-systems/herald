import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0

//PAUL: demagic all numbers and colors
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
    height: selector.height
    width: selector.width

    MouseArea {
        id: hoverHandler
        anchors.fill: bg
        hoverEnabled: true
        propagateComposedEvents: true

        onClicked: {
            maskShape.send(resultEmoji, takesModifier)
        }
    }

    background: Rectangle {
        id: bg
        opacity: parent.pressed || hoverHandler.containsMouse ? 1.0 : 0.0
        width: parent.width * 1.8
        height: parent.height * 1.8
        anchors.centerIn: parent
        color: lowlight
    }

    Text {
        id: selector
        opacity: 1.0
        anchors.centerIn: parent
        color: "white"
        font.pixelSize: 15
        text: resultEmoji
        elide: Text.ElideLeft
    }
}
