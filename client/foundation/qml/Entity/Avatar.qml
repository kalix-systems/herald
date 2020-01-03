import QtQuick 2.12
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as JS
import QtGraphicalEffects 1.0

Item {
    id: wrapperItem

    property bool isGroup: false
    property string pfpPath
    property color color
    property real size: CmnCfg.avatarSize
    property color textColor: CmnCfg.palette.iconFill
    property string initials

    // group avatars ar 4px smaller and have extra horizontal margin
    property int groupSize: size - 4

    height: size
    width: height

    Loader {
        id: iconLoader
        sourceComponent: pfpPath === "" ? textAvatar : imageAvatar
        anchors.fill: parent
    }

    Component {
        id: textAvatar
        Rectangle {
            height: isGroup ? groupSize : size
            width: height
            radius: isGroup ? 0 : width
            color: wrapperItem.color
            Text {
                text: initials
                font.bold: true
                font.pixelSize: ((initials.length > 1 ? 1.0 : 0.67)
                                 * parent.width) / initials.length
                anchors.centerIn: parent
                color: textColor
            }
        }
    }

    Component {
        id: imageAvatar
        Rectangle {
            color: CmnCfg.palette.offBlack
            height: isGroup ? groupSize : size
            width: height
            radius: isGroup ? 0 : width
            id: mask
            Image {
                source: JS.safeToQrcURI(pfpPath)
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
