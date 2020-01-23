import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
//import QtQuick.Dialogs 1.3
import "qrc:/imports"
import QtGraphicalEffects 1.0
import "../Common"

Page {
    id: galleryPage
    property int currentIndex
    property var imageAttachments
    property real imageScale: 1.0
    property real constrainedZoom: Math.max(0.5, Math.min(imageScale, 4.0))

    readonly property Component headerComponent: MessageInfoHeader {}

    // onClosed: galleryLoader.active = false
    background: Rectangle {
        id: background
        color: CmnCfg.palette.black
    }
    Component.onCompleted: loader.active = true
    Loader {
        id: loader
        anchors.fill: parent

        active: false
        sourceComponent: Item {
            anchors.fill: parent
            Row {
                id: buttonRowRight
                anchors.top: parent.top
                anchors.right: parent.right
                layoutDirection: Qt.RightToLeft
                height: 30
                spacing: CmnCfg.smallMargin

                AnimIconButton {
                    id: xIcon
                    imageSource: "qrc:/x-icon.svg"
                    icon.height: 30
                    icon.width: 30
                    color: CmnCfg.palette.white
                    z: galleryPage.z + 1
                    onTapped: {
                        mainView.pop()
                    }
                }

                AnimIconButton {
                    id: download
                    imageSource: "qrc:/download-icon.svg"
                    color: CmnCfg.palette.white
                    icon.height: 30
                    icon.width: 30
                    z: galleryPage.z + 1
                }
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

                AnimIconButton {
                    id: zoomIn
                    imageSource: "qrc:/zoom-in-icon.svg"
                    onTapped: zoomInFunction()
                    color: CmnCfg.palette.white
                    icon.height: 30
                    icon.width: 30
                    enabled: imageScale < 4.0
                    opacity: enabled ? 1.0 : 0.5
                    z: galleryPage.z + 1
                }
                AnimIconButton {
                    id: zoomOut
                    imageSource: "qrc:/zoom-out-icon.svg"
                    onTapped: {

                        galleryPage.imageScale -= 0.3
                        resizeContent()
                    }
                    color: CmnCfg.palette.white
                    icon.height: 30
                    icon.width: 30
                    enabled: imageScale > 0.5
                    opacity: enabled ? 1.0 : 0.5
                    z: galleryPage.z + 1
                }
            }

            AnimIconButton {
                id: next
                z: galleryPage.z + 1
                icon.height: 30
                icon.width: 30
                anchors.verticalCenter: flickable.verticalCenter
                anchors.left: flickable.right
                anchors.leftMargin: CmnCfg.smallMargin
                imageSource: "qrc:/forward-arrow-icon.svg"
                enabled: currentIndex !== imageAttachments.length - 1
                color: CmnCfg.palette.white
                opacity: enabled ? 1.0 : 0.5
                onTapped: {
                    galleryPage.imageScale = 1.0
                    galleryPage.currentIndex += 1
                    clipScroll.positionViewAtIndex(galleryPage.currentIndex,
                                                   ListView.Contain)
                }
            }

            AnimIconButton {
                id: back
                z: galleryPage.z + 1
                icon.height: 30
                icon.width: 30
                anchors.verticalCenter: flickable.verticalCenter
                anchors.right: flickable.left
                anchors.rightMargin: CmnCfg.smallMargin
                imageSource: "qrc:/back-arrow-icon.svg"
                enabled: currentIndex !== 0
                color: CmnCfg.palette.white
                opacity: enabled ? 1.0 : 0.5
                onTapped: {
                    galleryPage.imageScale = 1.0
                    galleryPage.currentIndex -= 1
                    clipScroll.positionViewAtIndex(galleryPage.currentIndex,
                                                   ListView.Contain)
                }
            }

            function zoomInFunction() {
                galleryPage.imageScale += 0.3
                resizeContent()
            }

            function resizeContent() {
                flickable.resizeContent(galleryPage.width * constrainedZoom,
                                        galleryPage.height * constrainedZoom,
                                        Qt.point(image.width / 2 + image.x,
                                                 image.height / 2 + image.y))
            }

            Flickable {
                id: flickable
                width: parent.width - 64
                height: parent.height - 120
                anchors.top: parent.top
                // anchors.topMargin: 40
                anchors.horizontalCenter: parent.horizontalCenter

                clip: true
                ScrollBar.vertical: ScrollBar {}
                ScrollBar.horizontal: ScrollBar {}
                contentHeight: height
                contentWidth: width
                contentItem.anchors.centerIn: (contentHeight
                                               < flickable.height) ? flickable : undefined
                boundsMovement: Flickable.StopAtBounds
                Component.onCompleted: print(width, height, image.width,
                                             image.height)
                Image {
                    id: image
                    source: "file:" + imageAttachments[currentIndex].path
                    fillMode: Image.PreserveAspectFit
                    anchors.fill: parent
                    property var dims: JSON.parse(
                                           Herald.utils.imageScaling(
                                               imageAttachments[index].path,
                                               parent.width))
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

                    galleryPage.imageScale += (pinch.scale - pinch.previousScale) * 1.2
                    flickable.resizeContent(
                                galleryPage.width * constrainedZoom,
                                galleryPage.height * constrainedZoom, pt)
                }

                onPinchFinished: {
                    flickable.returnToBounds()
                }
            }

            ListView {
                id: clipScroll
                width: parent.width
                height: 80
                clip: true
                orientation: Qt.Horizontal
                anchors.top: flickable.bottom
                anchors.topMargin: CmnCfg.largeMargin
                anchors.horizontalCenter: flickable.horizontalCenter
                model: imageAttachments
                ScrollBar.horizontal: ScrollBar {}
                spacing: CmnCfg.smallMargin
                delegate: Rectangle {
                    property var imageModel: model
                    height: 64
                    width: 64
                    clip: true
                    color: "transparent"
                    property var imageSource: "file:" + imageAttachments[index].path
                    Image {
                        id: clip
                        property var dims: JSON.parse(
                                               Herald.utils.imageScaling(
                                                   imageAttachments[index].path,
                                                   64))
                        sourceSize.width: dims.width
                        sourceSize.height: dims.height
                        anchors.centerIn: parent
                        source: parent.imageSource
                        fillMode: Image.PreserveAspectFit
                        ColorOverlay {
                            id: overlay
                            anchors.fill: parent
                            source: parent
                            visible: galleryPage.currentIndex !== index
                            color: CmnCfg.palette.black
                            opacity: 0.7
                            smooth: true
                        }
                    }
                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            galleryPage.currentIndex = index
                            imageScale = 1.0
                            flickable.contentHeight = flickable.height
                            flickable.contentWidth = flickable.width
                        }
                        z: galleryPage.z + 1
                    }
                }
            }
        }
    }
}
