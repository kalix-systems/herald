import QtQuick 2.12
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as JS
import QtGraphicalEffects 1.0

Item {
    id: wrapperItem
    property string pfpUrl
    property bool groupAvatar: false
    property color color
    property color textColor: CmnCfg.palette.iconFill
    property string initials
    property real avatarHeight: parent.avatarHeight

    Loader {
        id: iconLoader
        sourceComponent: pfpUrl === "" ? textAvatar : imageAvatar
        anchors.fill: parent
    }

    Component {
        id: textAvatar
        Rectangle {
            height: parent.height
            width: height
            radius: groupAvatar ? 0 : width
            color: wrapperItem.color
            Text {
                text: initials
                font.bold: true
                font.pixelSize: 2 / 3 * parent.width / initials.length
                anchors.centerIn: parent
                color: textColor
            }
        }
    }

    Component {
        id: imageAvatar
        Rectangle {
            color: CmnCfg.palette.secondaryColor
            height: parent.height
            width: height
            radius: groupAvatar ? 0 : width
            id: mask
            Image {
                source: JS.safeToQrcURI(pfpUrl)
                anchors.fill: mask
                layer.enabled: true
                layer.effect: OpacityMask {
                    maskSource: mask
                }
                clip: true
                mipmap: true
            }
        }
    }
}
