import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0

//PAUL: demagic all numbers and colors
Button {
    property bool takesModifier
    property string baseEmoji
    // consider this a private field
    property string resultEmoji: takesModifier ? baseEmoji + CmnCfg.skinSwatchList[CmnCfg.skinSwatchIndex] : baseEmoji
    property color lowlight: CmnCfg.palette.darkGrey
    onClicked: maskShape.send(resultEmoji)
    height: selector.height
    width: selector.width

    MouseArea {
        id: hoverHandler
        anchors.fill: parent
        hoverEnabled: true
        propagateComposedEvents: true
        onClicked: mouse.accepted = false
        onReleased: mouse.accepted = false
        onPressed: mouse.accepted = false
        onPressAndHold: mouse.accepted = false
    }

    background: Rectangle {
        id: bg
        opacity: parent.pressed || hoverHandler.containsMouse ? 1.0 : 0.0
        anchors.fill: parent
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
