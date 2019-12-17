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

    Row {
        id: buttonRowRight
        anchors.top: parent.top
        anchors.right: parent.right
        layoutDirection: Qt.RightToLeft
        height: 30
        spacing: CmnCfg.smallMargin

        ButtonForm {
            id: xIcon
            source: "qrc:/x-icon.svg"
            icon.height: 30
            icon.width: 30
            fill: CmnCfg.palette.white
            z: galleryPopup.z + 1
            onClicked: {
                galleryLoader.active = false
                galleryPopup.close()
            }
        }

        ButtonForm {
            id: download
            source: "qrc:/download-icon.svg"
            fill: CmnCfg.palette.white
            icon.height: 30
            icon.width: 30
            z: galleryPopup.z + 1
            onClicked: downloadImage.open()
        }
    }

    FileDialog {
        id: downloadImage
        selectExisting: false
        selectFolder: true
        selectMultiple: false
        folder: StandardPaths.writableLocation(StandardPaths.DesktopLocation)
        onAccepted: Herald.utils.saveFile(imageAttachments[currentIndex].path,
                                          fileUrl)
    }

    Label {
        anchors.top: parent.top
        anchors.left: buttonRowLeft.right
        anchors.right: buttonRowRight.left
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.rightMargin: CmnCfg.smallMargin
        text: imageAttachments[currentIndex].name
        font.pixelSize: 20
        font.family: CmnCfg.chatFont.name
        color: CmnCfg.palette.white
        elide: Text.ElideMiddle
        horizontalAlignment: Text.AlignHCenter
    }

    Row {
        id: buttonRowLeft
        anchors.top: parent.top
        anchors.left: parent.left
        height: 30
        spacing: CmnCfg.smallMargin

        ButtonForm {
            id: zoomIn
            source: "qrc:/plus-icon.svg"
            action: zoomAction
            fill: CmnCfg.palette.white
            icon.height: 30
            icon.width: 30
            z: galleryPopup.z + 1
        }
        ButtonForm {
            id: zoomOut
            source: "qrc:/minus-icon.svg"
            action: zoomOutAction
            fill: CmnCfg.palette.white
            icon.height: 30
            icon.width: 30
            z: galleryPopup.z + 1
        }
    }

    ButtonForm {
        id: next
        z: galleryPopup.z + 1
        icon.height: 30
        icon.width: 30
        anchors.verticalCenter: flickable.verticalCenter
        anchors.right: parent.right
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
        height: parent.height - 110
        anchors.top: parent.top
        anchors.topMargin: 30
        anchors.horizontalCenter: parent.horizontalCenter

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
    ImageClipRow {
        id: clipScroll
    }
}
