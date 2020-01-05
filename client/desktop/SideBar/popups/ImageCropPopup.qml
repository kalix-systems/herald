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
    property url imageSource
    property real maxSize: 300
    property int maxWindowSize: 400
    property int minSize: Math.round(maxSize / 6)
    color: CmnCfg.palette.black

    Row {
        anchors {
            bottom: parent.bottom
            horizontalCenter: parent.horizontalCenter
            margins: CmnCfg.defaultMargin
        }
        spacing: CmnCfg.defaultMargin
        TextButton {
            z: image.z + 1
            text: "Set"
            onClicked: {
                const picture = {
                    "width": Math.round(clipRect.width),
                    "height": Math.round(clipRect.height),
                    "x": Math.round(clipRect.x),
                    "y": Math.round(clipRect.y),
                    "path": Herald.utils.stripUrlPrefix(imageSource)
                }
                Herald.config.setProfilePicture(JSON.stringify(picture))
                cropWindow.close()
            }
        }

        TextButton {
            z: image.z + 1
            text: "Cancel"
            onClicked: cropWindow.close()
        }
    }

    width: 400
    height: 400

    Image {
        id: image
        onSourceChanged: {
            if (source !== undefined)
                dims = JSON.parse(Herald.utils.imageScaling(
                                      Herald.utils.stripUrlPrefix(
                                          image.source), 300))
        }
        property var dims
        anchors.centerIn: parent
        source: imageSource

        sourceSize.height: dims === undefined ? 0 : dims.height
        sourceSize.width: dims === undefined ? 0 : dims.width
        width: sourceSize.width
        height: sourceSize.height

        Rectangle {
            id: top
            anchors.top: image.top
            anchors.bottom: clipRect.top
            anchors.left: left.right
            anchors.right: image.right
            color: CmnCfg.palette.black
            opacity: 0.5
        }

        Rectangle {
            id: right
            anchors.left: clipRect.right
            anchors.right: image.right
            anchors.top: top.bottom
            anchors.bottom: image.bottom
            color: CmnCfg.palette.black
            opacity: 0.5
        }

        Rectangle {
            id: left
            anchors.right: clipRect.left
            anchors.left: image.left
            anchors.bottom: bottom.top
            anchors.top: image.top
            color: CmnCfg.palette.black
            opacity: 0.5
        }

        Rectangle {
            id: bottom
            anchors.bottom: image.bottom
            anchors.top: clipRect.bottom
            anchors.right: right.left
            anchors.left: image.left
            color: CmnCfg.palette.black
            opacity: 0.5
        }

        Rectangle {
            id: clipRect
            width: Math.min(image.width, image.height)
            height: width
            color: "transparent"
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
                    clipRect.anchors.centerIn = null
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
            cursorShape: Qt.SizeFDiagCursor
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
