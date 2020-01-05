import QtQuick 2.14
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.13
import QtQuick.Window 2.14
import LibHerald 1.0
import "qrc:/imports"
import "../../common" as Common

Window {
    id: cropWindow
    property real imageWidth
    property real imageHeight
    property url imageSource
    property real aspectRatio: imageWidth / imageHeight
    property real maxSize: Math.min(imageWidth, imageHeight)
    property int maxWindowSize: 400
    property int minSize: Math.round(maxSize / 6)

    Button {
        anchors.top: parent.top
        z: image.z + 1
        text: "set"
        onClicked: {
            const picture = {
                "width": Math.round(clipRect.width),
                "height": Math.round(clipRect.height),
                "x": Math.round(clipRect.x),
                "y": Math.round(clipRect.y),
                "path": Herald.utils.stripUrlPrefix(imageSource)
            }

            Herald.config.setProfilePicture(JSON.stringify(picture))
        }
    }

    width: imageWidth + 100 //(aspectRatio > 1) ? maxWindowSize : maxWindowSize * aspectRatio
    height: imageHeight + 100 //(aspectRatio > 1) ? maxWindowSize / aspectRatio : maxWindowSize

    Image {
        id: image
        anchors.centerIn: parent
        source: imageSource

        fillMode: Image.PreserveAspectFit

        Rectangle {
            id: clipRect
            width: Math.min(imageWidth, imageHeight)
            height: width
            color: CmnCfg.palette.darkGrey
            opacity: 0.5
            anchors.centerIn: parent

            onWidthChanged: {
                clipRect.anchors.centerIn = null
                if ((x + width) > image.width) {
                    x = image.width - width
                }
                if (x < 0) {
                    x = 0
                }
            }

            onHeightChanged: {
                clipRect.anchors.centerIn = null
                if ((y + height) > image.height) {
                    y = image.height - height
                }
                if (y < 0) {
                    y = 0
                }
            }

            MouseArea {
                width: parent.width
                height: parent.height
                anchors.centerIn: parent
                drag.target: parent
                drag.axis: Drag.XandYAxis
                drag.minimumX: 0
                drag.minimumY: 0
                drag.maximumX: image.width - clipRect.width
                drag.maximumY: image.height - clipRect.height

                onPressed: {
                    clipRect.color = CmnCfg.palette.offBlack
                    clipRect.anchors.centerIn = null
                }
                onReleased: {
                    clipRect.color = CmnCfg.palette.darkGrey
                }
            }
        }

        Rectangle {
            id: target
            anchors.horizontalCenter: clipRect.right
            anchors.verticalCenter: clipRect.bottom
            color: CmnCfg.palette.offBlack
            opacity: 1.0
            height: 10
            width: height
        }

        MouseArea {
            parent: image
            anchors.fill: target
            drag.target: target
            drag.axis: Drag.XandYAxis
            drag.maximumX: image.width - clipRect.x
            drag.maximumY: image.height - clipRect.y

            onMouseXChanged: if (drag.active) {
                                 clipRect.width += Math.min(
                                             mouseX, maxSize - clipRect.width)
                                 if (clipRect.width < minSize) {
                                     clipRect.width = minSize
                                 }

                                 clipRect.height = clipRect.width
                             }
            onMouseYChanged: if (drag.active) {
                                 clipRect.height += Math.min(
                                             mouseY, maxSize - clipRect.width)
                                 if (clipRect.height < minSize) {
                                     clipRect.height = minSize
                                 }
                                 clipRect.width = clipRect.height
                             }
        }
    }
}
