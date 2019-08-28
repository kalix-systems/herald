import QtQuick 2.13
import LibHerald 1.0
import QtGraphicalEffects 1.0
import QtQuick.Controls 2.12

/// --- displays a list of contacts
Row {
    property string displayName: ""
    property string pfpUrl: ""
    property int colorHash: 0
    property int shapeEnum: 0 /// { individual, group ... }
    property int size: 0 /// the size of the avatar, width and height
    spacing: QmlCfg.margin

    ///--- Circle with initial
    leftPadding: QmlCfg.margin
    anchors.verticalCenter: parent.verticalCenter

    Loader {
        width: size
        height: size
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
        text: displayName
        font.bold: true
        anchors.verticalCenter: parent.verticalCenter
    }

    ///--- potential avatar components
    Component {
        id: initialAvatar
        Rectangle {
            width: size
            height: size
            anchors.verticalCenter: parent.verticalCenter
            color: QmlCfg.avatarColors[colorHash]
            radius: if (shapeEnum === 0) {
                        width
                    } else {
                        0
                    }
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

    Component {
        id: imageAvatar
        Item {
            Rectangle {
                color: QmlCfg.palette.mainColor
                width: size
                height: size
                radius: if (shapeEnum === 0) {
                            width
                        } else {
                            0
                        }
                id: mask
            }
            Image {
                source: "file:" + pfpUrl
                anchors.fill: mask
                layer.enabled: true
                layer.effect: OpacityMask {
                    maskSource: mask
                }
                clip: true
                asynchronous: true
                mipmap: true
            }
        }
    }
}
