import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3

Popup {
    id: galleryPopup
    property int currentIndex: parent.currentIndex
    property var imageAttachments: parent.imageAttachments
    property real imageScale: 1.0
    readonly property var reset: function () {//should reset the window
    }
    onClosed: galleryLoader.active = false

    height: parent.height
    width: parent.width
    anchors.centerIn: parent
    background: Rectangle {
        color: "black"
    }
    Action {
        shortcut: StandardKey.MoveToNextChar
        onTriggered: flickable.contentX += flickable.contentWidth * 0.1
    }

    Action {
        shortcut: StandardKey.MoveToPreviousChar
        onTriggered: flickable.contentX -= flickable.contentWidth * 0.1
    }

    Action {
        shortcut: StandardKey.MoveToPreviousLine
        onTriggered: flickable.contentY -= flickable.contentHeight * 0.1
    }

    Action {
        shortcut: StandardKey.MoveToNextLine
        onTriggered: flickable.contentY += flickable.contentHeight * 0.1
    }

    Action {
        id: zoomAction
        shortcut: StandardKey.ZoomIn
        onTriggered: {
            galleryPopup.scale += 0.3
            flickable.resizeContent(galleryPopup.width * galleryPopup.scale,
                                    galleryPopup.height * galleryPopup.scale,
                                    Qt.point(image.width / 2 + image.x,
                                             image.height / 2 + image.y))
        }
    }

    Action {
        id: zoomOutAction
        shortcut: StandardKey.ZoomOut
        onTriggered: {
            galleryPopup.scale -= 0.3
            flickable.resizeContent(galleryPopup.width * galleryPopup.scale,
                                    galleryPopup.height * galleryPopup.scale,
                                    Qt.point(image.width / 2 + image.x,
                                             image.height / 2 + image.y))
        }
    }

    Flickable {
        id: flickable
        width: parent.width
        height: parent.height - 80
        anchors.top: parent.top
        clip: true
        ScrollBar.vertical: ScrollBar {}
        ScrollBar.horizontal: ScrollBar {}
        contentHeight: height
        contentWidth: width
        contentItem.anchors.centerIn: (contentHeight < flickable.height) ? flickable : undefined

        Image {
            id: image
            source: "file:" + imageAttachments[currentIndex].path
            fillMode: Image.PreserveAspectFit
            anchors.fill: parent
            mipmap: true
            asynchronous: true
        }
    }

    PinchArea {
        id: pinchArea
        anchors.fill: parent
        onPinchUpdated: {
            galleryPopup.imageScale += (pinch.scale - pinch.previousScale) * 1.2
            flickable.resizeContent(
                        galleryPopup.width * galleryPopup.imageScale,
                        galleryPopup.height * galleryPopup.imageScale,
                        pinch.center)
        }
    }

    Flickable {
        width: parent.width
        anchors.top: flickable.bottom
        anchors.topMargin: CmnCfg.smallMargin
        anchors.horizontalCenter: flickable.horizontalCenter
        Row {
            height: parent.height
            spacing: CmnCfg.smallMargin
            Repeater {
                model: imageAttachments
                delegate: Rectangle {
                    property var imageModel: model
                    height: 64
                    width: 64
                    clip: true
                    property real aspectRatio: imageAttachments[index].width
                                               / imageAttachments[index].height
                    property var imageSource: "file:" + imageAttachments[index].path
                    Image {
                        id: clip
                        sourceSize.width: parent.aspectRatio < 1 ? 64 : 64 * parent.aspectRatio
                        sourceSize.height: parent.aspectRatio < 1 ? 64 / parent.aspectRatio : 64
                        anchors.centerIn: parent
                        source: parent.imageSource
                    }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            currentIndex = index
                            imageScale = 1.0
                            flickable.contentHeight = flickable.height
                            flickable.contentWidth = flickable.width
                        }
                    }
                }
            }
        }
    }
}
