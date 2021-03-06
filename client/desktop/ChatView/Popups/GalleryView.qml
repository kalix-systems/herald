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
    property real constrainedZoom: Math.max(0.5, Math.min(imageScale, 4.0))
    onClosed: galleryLoader.active = false

    height: root.height
    width: root.width
    anchors.centerIn: parent

    enter: Transition {

        NumberAnimation {
            property: "opacity"
            from: 0.0
            to: 1.0
            duration: 200
            easing.type: Easing.OutQuad
        }
    }
    exit: Transition {

        NumberAnimation {
            id: exitAnimation
            property: "opacity"
            from: 1.0
            to: 0.0
            duration: 200
            easing.type: Easing.OutQuad
        }
    }

    background: Rectangle {
        id: background
        color: CmnCfg.palette.black
    }

    Row {
        id: buttonRowRight
        anchors.top: parent.top
        anchors.right: parent.right
        layoutDirection: Qt.RightToLeft
        height: 30
        spacing: CmnCfg.smallMargin

        IconButton {
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

        IconButton {
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

        IconButton {
            id: zoomIn
            source: "qrc:/zoom-in-icon.svg"
            action: zoomAction
            fill: CmnCfg.palette.white
            icon.height: 30
            icon.width: 30
            enabled: imageScale < 4.0
            opacity: enabled ? 1.0 : 0.5
            z: galleryPopup.z + 1
        }
        IconButton {
            id: zoomOut
            source: "qrc:/zoom-out-icon.svg"
            action: zoomOutAction
            fill: CmnCfg.palette.white
            icon.height: 30
            icon.width: 30
            enabled: imageScale > 0.5
            opacity: enabled ? 1.0 : 0.5
            z: galleryPopup.z + 1
        }
    }

    IconButton {
        id: next
        z: galleryPopup.z + 1
        icon.height: 30
        icon.width: 30
        anchors.verticalCenter: flickable.verticalCenter
        anchors.left: flickable.right
        anchors.leftMargin: CmnCfg.smallMargin
        source: "qrc:/forward-arrow-icon.svg"
        enabled: currentIndex !== imageAttachments.length - 1
        fill: CmnCfg.palette.white
        opacity: enabled ? 1.0 : 0.5
        onClicked: {
            galleryPopup.imageScale = 1.0
            galleryView.currentIndex += 1
            clipScroll.positionViewAtIndex(galleryView.currentIndex,
                                           ListView.Contain)
        }
    }

    IconButton {
        id: back
        z: galleryPopup.z + 1
        icon.height: 30
        icon.width: 30
        anchors.verticalCenter: flickable.verticalCenter
        anchors.right: flickable.left
        anchors.rightMargin: CmnCfg.smallMargin
        source: "qrc:/back-arrow-icon.svg"
        enabled: currentIndex !== 0
        fill: CmnCfg.palette.white
        opacity: enabled ? 1.0 : 0.5
        onClicked: {
            galleryPopup.imageScale = 1.0
            galleryView.currentIndex -= 1
            clipScroll.positionViewAtIndex(galleryView.currentIndex,
                                           ListView.Contain)
        }
    }

    Action {
        shortcut: StandardKey.MoveToNextChar
        onTriggered: if (galleryView.currentIndex < imageAttachments.length - 1)
                         galleryView.currentIndex += 1
    }

    Action {
        shortcut: StandardKey.MoveToPreviousChar
        onTriggered: if (galleryView.currentIndex > 0)
                         galleryView.currentIndex -= 1
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
        onTriggered: zoomInFunction()
    }

    Shortcut {
        sequence: "Ctrl+="
        onActivated: zoomInFunction()
    }

    Action {
        id: zoomOutAction
        shortcut: StandardKey.ZoomOut
        onTriggered: {
            galleryPopup.imageScale -= 0.3
            resizeContent()
        }
    }

    function zoomInFunction() {
        galleryPopup.imageScale += 0.3
        resizeContent()
    }

    function resizeContent() {
        flickable.resizeContent(galleryPopup.width * constrainedZoom,
                                galleryPopup.height * constrainedZoom,
                                Qt.point(image.width / 2 + image.x,
                                         image.height / 2 + image.y))
    }

    Flickable {
        id: flickable
        width: parent.width - 64
        height: parent.height - 120
        anchors.top: parent.top
        anchors.topMargin: 40
        anchors.horizontalCenter: parent.horizontalCenter

        clip: true
        ScrollBar.vertical: ScrollBar {}
        ScrollBar.horizontal: ScrollBar {}
        contentHeight: height
        contentWidth: width
        contentItem.anchors.centerIn: (contentHeight < flickable.height) ? flickable : undefined
        boundsMovement: Flickable.StopAtBounds

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

        property point pt
        property real flickableStartX
        property real flickableStartY

        onPinchUpdated: {
            pt = Qt.point(flickable.contentWidth / 2,
                          flickable.contentHeight / 2)

            galleryPopup.imageScale += (pinch.scale - pinch.previousScale) * 1.2
            flickable.resizeContent(galleryPopup.width * constrainedZoom,
                                    galleryPopup.height * constrainedZoom, pt)
        }

        onPinchFinished: {
            flickable.returnToBounds()
        }
    }
    ImageClipRow {
        id: clipScroll
    }
}
