import QtQuick 2.13
import LibHerald 1.0
import QtGraphicalEffects 1.0
import QtQuick.Controls 2.13

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping

/// TS: All propertyies should be set at once inside typescript,
/// it should receive reference to the contact item. and handle it from there

/// --- displays a list of contacts
Row {
    // Note: empty string or undefined? whatever leads to less coercions
    property string displayName: ""
    property string pfpUrl: ""
    property int colorHash: 0
    // TS: this enum should be defined in TS
    property int shapeEnum: 0 /// { individual, group ... }
    property int size: 0 /// the size of the avatar, width and height
    // TS: Note: This should be a path or border element in the future,
    // Selected by a TS function that knows about shapeEnum
    property int shape: if (shapeEnum === 0) {
                            size
                        } else {
                            0
                        }

    property bool labeled: true /// whether or not to show the name
    // NOTE: make a property in QMLCFG call padding. it is probably just 10
    spacing: QmlCfg.margin

    ///--- Circle with initial
    leftPadding: QmlCfg.margin
    anchors.verticalCenter: parent.verticalCenter

    Loader {
        width: size
        height: size
        /// TS: this should have a specific TS function to set these values
        sourceComponent: {
            if (displayName === "")
                return undefined
            if (pfpUrl !== "")
                return imageAvatar
            else
                return initialAvatar
        }
    }


   Text {
      visible: labeled
      text: displayName
      font.bold: true
      anchors.verticalCenter: parent.verticalCenter
     }

    ///--- potential avatar components
    /// NPB: looks very clunky and bad by default, choose fonts, finalize design, maybe don't do
    /// what every other chat app does for this? are there easier to track avatars out there?
    Component {
        id: initialAvatar
        Rectangle {
            width: size
            height: size
            anchors.verticalCenter: parent.verticalCenter
            color: QmlCfg.avatarColors[colorHash]
            // Note:
            radius: shape
            ///---- initial
            Text {
                text: qsTr(displayName[0].toUpperCase())
                font.bold: true
                color: "white"
                anchors.centerIn: parent
                font.pixelSize: size
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
                // TS : this URI-ification should be in TS or rust
                source: "file:" + pfpUrl
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
