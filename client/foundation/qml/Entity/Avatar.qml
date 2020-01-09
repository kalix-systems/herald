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
            border.width: isGroup ? 1 : 0
            border.color: CmnCfg.palette.white
            Text {
                text: initials
                font.weight: Font.DemiBold
                font.family: CmnCfg.chatFont.name
                font.pixelSize: ((initials.length > 1 ? 1.0 : 0.67)
                                 * parent.width) / initials.length
                anchors.centerIn: parent
                color: textColor
            }
        }
    }

    Component {
        id: imageAvatar
        Item {
            width: border.width
            height: border.height
            Rectangle {
                color: !isGroup ? wrapperItem.color : CmnCfg.palette.white
                height: isGroup ? groupSize : size
                width: height
                radius: isGroup ? 0 : width
                id: mask
                clip: true
                anchors.centerIn: parent

                Image {
                    id: image
                    source: JS.safeToQrcURI(pfpPath)
                    anchors.fill: mask
                    layer.enabled: true
                    layer.mipmap: true
                    layer.effect: OpacityMask {
                        maskSource: mask
                    }
                    layer.smooth: true
                    mipmap: true
                }
            }

            Rectangle {
                id: border
                anchors.centerIn: parent
                width: isGroup ? mask.width + 2 : mask.width + 4
                height: width
                radius: !isGroup ? width : 0
                color: !isGroup ? wrapperItem.color : CmnCfg.palette.white
                z: parent.z - 1
            }
        }
    }
}
