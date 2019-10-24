import QtQuick 2.12
import LibHerald 1.0

Item {
    property string pfpUrl
    property bool groupAvatar: false
    property color color
    property string initials

    Image {
        source: pfpUrl
    }

    Rectangle {
        height: CmnCfg.avatarHeight
        width: height
        radius: groupAvatar ? 0 : width
        color: parent.color
        Text {
            text: initials
            font.bold: true
            font.pixelSize: parent.width / initials.length
            anchors.centerIn: parent
            color: CmnCfg.palette.iconFill
        }
    }

    Image {
        source: pfpUrl
    }
}
