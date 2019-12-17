import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "qrc:/imports"
import QtGraphicalEffects 1.0

Popup {
    id: galleryPopup
    property int currentIndex: parent.currentIndex
    property var imageAttachments: parent.imageAttachments
    property real imageScale: 1.0
    readonly property var reset: function () {//should reset the window
    }
    onClosed: galleryLoader.active = false

    height: root.height
    width: root.width
    anchors.centerIn: parent
    background: Rectangle {
        color: "black"
    }

    ButtonForm {
        id: xIcon
        source: "qrc:/x-icon.svg"
        icon.height: 30
        icon.width: 30
        anchors.top: parent.top
        anchors.right: parent.right
        fill: CmnCfg.palette.white
        z: galleryPopup.z + 1
        onClicked: {
            galleryLoader.active = false
            galleryPopup.close()
        }
    }

    ButtonForm {
        id: next
        z: galleryPopup.z + 1
        icon.height: 30
        icon.width: 30
        anchors.verticalCenter: flickable.verticalCenter
        anchors.horizontalCenter: xIcon.horizontalCenter
        source: "qrc:/forward-arrow-icon.svg"
        enabled: currentIndex !== imageAttachments.length - 1
        fill: CmnCfg.palette.white
        opacity: enabled ? 1.0 : 0.5
        onClicked: {
            galleryView.currentIndex += 1
            clipScroll.positionViewAtIndex(galleryView.currentIndex,
                                           ListView.Contain)
        }
    }

    ButtonForm {
        id: back
        z: galleryPopup.z + 1
        icon.height: 30
        icon.width: 30
        anchors.verticalCenter: flickable.verticalCenter
        anchors.left: parent.left
        source: "qrc:/back-arrow-icon.svg"
        enabled: currentIndex !== 0
        fill: CmnCfg.palette.white
        opacity: enabled ? 1.0 : 0.5
        onClicked: {
            galleryView.currentIndex -= 1
            clipScroll.positionViewAtIndex(galleryView.currentIndex,
                                           ListView.Contain)
        }
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
            galleryPopup.imageScale += 0.3
            flickable.resizeContent(
                        galleryPopup.width * galleryPopup.imageScale,
                        galleryPopup.height * galleryPopup.imageScale,
                        Qt.point(image.width / 2 + image.x,
                                 image.height / 2 + image.y))
        }
    }

    Action {
        id: zoomOutAction
        shortcut: StandardKey.ZoomOut
        onTriggered: {
            galleryPopup.imageScale -= 0.3
            flickable.resizeContent(
                        galleryPopup.width * galleryPopup.imageScale,
                        galleryPopup.height * galleryPopup.imageScale,
                        Qt.point(image.width / 2 + image.x,
                                 image.height / 2 + image.y))
        }
    }

    Flickable {
        id: flickable
        width: parent.width - 50
        height: parent.height - 80
        anchors.top: parent.top
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.rightMargin: CmnCfg.smallMargin

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

    ListView {
        id: clipScroll
        width: parent.width
        height: 80
        Component.onCompleted: print(height, width)
        clip: true
        orientation: Qt.Horizontal
        anchors.top: flickable.bottom
        anchors.topMargin: CmnCfg.smallMargin
        anchors.horizontalCenter: flickable.horizontalCenter
        model: imageAttachments
        spacing: CmnCfg.smallMargin
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
                ColorOverlay {
                    id: overlay
                    anchors.fill: parent
                    source: parent
                    visible: galleryPopup.currentIndex !== index
                    color: CmnCfg.palette.black
                    opacity: 0.7
                    smooth: true
                }
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
