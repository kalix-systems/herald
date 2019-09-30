import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0

Button {
    property string    imageSource: ""
    property int       anchorIndex
    property ListView  list
    property color lowlight: "light gray"

    onClicked: {
//        list.positionViewAtIndex(anchorIndex)
    }

    height: selector.height+5
     width: selector.width+5

    background: Rectangle {
        id: bg
        radius: 5
        opacity: parent.pressed ? 1.0 : 0.0
        anchors.fill: parent
        color: lowlight
    }

    Image {
        id: selector
        opacity: 1.0
        source: imageSource
        sourceSize: Qt.size(24,24)
        height: 17
        width: height
        anchors.centerIn: parent
    }

}
