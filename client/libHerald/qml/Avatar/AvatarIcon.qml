import QtQuick 2.12

Item {
    property string pfpUrl
    property bool groupAvatar: false
    property color color

    Image {
        source: pfpUrl
    }

    Rectangle {
        anchors.fill: parent
        radius: groupAvatar ? 0 : width
        color: parent.color
    }
}
