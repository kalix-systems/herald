import QtQuick 2.14
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.13
import QtQuick.Window 2.2
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
    width: (aspectRatio > 1) ? maxWindowSize : maxWindowSize / aspectRatio
    height: (aspectRatio > 1) ? maxWindowSize * aspectRatio : maxWindowSize

    Image {
        anchors.centerIn: parent
        source: imageSource
        fillMode: Image.PreserveAspectFit
        Item {
            id: wrapperItem
            anchors.centerIn: parent

            //  clipWidth: Math.min(cropWindow.imageHeight, cropWindow.imageWidth)
            //  clipHeight: clipWidth
            width: Math.min(cropWindow.imageHeight, cropWindow.imageWidth)
            height: width

            Rectangle {
                id: clipRect
                width: parent.width
                height: width
                color: CmnCfg.palette.darkGrey
                opacity: 0.5
                anchors.centerIn: parent
            }

            Rectangle {
                id: target
                anchors.horizontalCenter: clipRect.right
                anchors.verticalCenter: clipRect.bottom
                color: CmnCfg.palette.offBlack
                opacity: 1.0
                height: 10
                width: height

                MouseArea {
                    anchors.fill: parent
                    drag.target: parent
                    drag.axis: Drag.XandYAxis
                    onMouseXChanged: if (drag.active) {
                                         wrapperItem.width += mouseX
                                         clipRect.x += mouseX
                                     }
                    onMouseYChanged: if (drag.active) {
                                         wrapperItem.height += mouseY
                                         clipRect.y += mouseY
                                     }
                }
            }

            MouseArea {
                width: parent.width
                height: parent.height
                anchors.centerIn: parent
                drag.target: wrapperItem
                drag.axis: Drag.XandYAxis

                onPressed: {
                    clipRect.color = CmnCfg.palette.offBlack
                    wrapperItem.anchors.centerIn = null
                }
                onReleased: {
                    clipRect.color = CmnCfg.palette.darkGrey
                }
            }
        }
    }
}
