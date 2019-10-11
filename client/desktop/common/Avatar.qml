import QtQuick 2.13
import LibHerald 1.0
import QtGraphicalEffects 1.0
import QtQuick.Controls 2.13
import "Avatar.mjs" as JS
import "utils.mjs" as Utils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping

/// --- displays a list of contacts
Row {
    id: wrapperRow
    property string avatarLabel: ""
    property string username: ""
    property string secondaryText: ""
    property string pfpUrl: ""
    property int colorHash: 0
    property int labelGap: 10.0
    property int shapeEnum: 0 /// { individual, group ... }
    property int size: 0 /// the size of the avatar, width and height
    property int shape: JS.avatarShape(shapeEnum, this)
    property bool labeled: true /// whether or not to show the name
    property bool isDefault: true

    spacing: QmlCfg.padding

    ///--- Circle with initial
    leftPadding: QmlCfg.margin
    anchors.verticalCenter: parent.verticalCenter
    anchors.top: parent.top
    Loader {
        id: avatarLoader
        width: size
        height: size
        sourceComponent: JS.avatarSource(avatarLabel, pfpUrl, imageAvatar,
                                         initialAvatar)
        anchors.verticalCenter: parent.verticalCenter
    }
    // TODO : this seems kinda like a seperate component at this point.
    Column {
        id: textCol
        anchors {
            topMargin: QmlCfg.margin
            verticalCenter: parent.verticalCenter
            top: labelGap === 0 ? undefined : wrapperRow.top
        }

        spacing: labelGap

        Text {
            id: displayName
            visible: labeled
            text: avatarLabel
            font.bold: true
            //is white instead of palette maincolor bc shld be white regardless of theme
            color: if (!!!isDefault) {
                       QmlCfg.palette.iconFill
                   } else {
                       QmlCfg.palette.mainTextColor
                   }
        }

        Text {
            id: userName
            visible: labeled
            text: secondaryText
            // is white instead of palette maincolor bc shld be white regardless of theme
            color: if (!!!isDefault) {
                       QmlCfg.palette.iconFill
                   } else {
                       QmlCfg.palette.secondaryTextColor
                   }
            elide: Text.ElideRight
        }
    }

    ///--- potential avatar components
    /// NPB: looks very clunky and bad by default, choose fonts, finalize design, maybe don't do
    /// what every other chat app does for this? are there easier to track avatars out there?
    Component {
        id: initialAvatar
        Rectangle {
            id: avatarRect
            width: size
            height: size
            anchors.verticalCenter: parent.verticalCenter

            //is white instead of palette maincolor bc shld be white regardless of theme
            readonly property color startColor: !!!isDefault ? QmlCfg.palette.iconFill : QmlCfg.avatarColors[colorHash]
            color: startColor
            // Note:
            radius: shape
            ///---- initial
            Text {
                text: qsTr(avatarLabel[0].toUpperCase())
                font.bold: true
                color: if (!isDefault) {
                           QmlCfg.avatarColors[colorHash]
                       } else {
                           QmlCfg.palette.iconFill
                       }
                anchors.centerIn: parent
                font.pixelSize: size * 2 / 3
            }
        }
    }

    ///--- image compoenent
    Component {
        id: imageAvatar
        Item {
            Rectangle {
                color: QmlCfg.palette.mainColor
                width: size
                height: size
                radius: shape
                id: mask
            }
            Image {
                source: Utils.safeToQrcURI(pfpUrl)
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
