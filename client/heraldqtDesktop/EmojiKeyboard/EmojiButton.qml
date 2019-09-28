import QtQuick 2.13
import QtQuick.Controls 2.13

Button {
    property string emoji: "👍"
    property color lowlight: "light gray"

    onClicked: {
        maskShape.send(emoji)
    }

    height: selector.height + 3
    width: selector.width + 5

    background: Rectangle {
        id: bg
        radius: 5
        opacity: parent.pressed ? 1.0 : 0.0
        anchors.fill: parent
        color: lowlight
    }

    Text {
        id: selector
        opacity: 1.0
        anchors.centerIn: parent
        font.pixelSize: 20
        text: qsTr(emoji)
    }

}
