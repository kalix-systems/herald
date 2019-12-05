import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick.Window 2.13

Window {
    id: imageWindow
    property real scale: 1.0
    property bool freeScroll: scale === 1.0
    property int index: 0
    property Attachments sourceAtc
    title: sourceAtc !== null ? sourceAtc.attachmentPath(index).substring(
                                    sourceAtc.attachmentPath(index).lastIndexOf(
                                        '/') + 1) : ""

    width: Math.min(image.sourceSize.width, 750)
    height: Math.min(image.sourceSize.height, 500)
    minimumWidth: 350
    minimumHeight: 150

    Row {
        id: controls
        z: CmnCfg.overlayZ
        spacing: CmnCfg.smallMargin
        padding: CmnCfg.smallMargin
        anchors {
            right: imageWindow.right
            top: imageWindow.top
        }
        Button {
            text: "+"
            font.bold: true
            font.pointSize: 20
            width: 50
            onClicked: {
                imageWindow.scale += 0.3
                flickable.resizeContent(imageWindow.width * imageWindow.scale,
                                        imageWindow.height * imageWindow.scale,
                                        Qt.point(image.width / 2 + image.x,
                                                 image.height / 2 + image.y))
            }
        }
        Button {
            text: "-"
            font.bold: true
            font.pointSize: 20
            width: 50
            onClicked: {
                imageWindow.scale -= 0.3
                flickable.resizeContent(imageWindow.width * imageWindow.scale,
                                        imageWindow.height * imageWindow.scale,
                                        Qt.point(image.width / 2 + image.x,
                                                 image.height / 2 + image.y))
            }
        }
        Button {
            text: "↓"
            font.bold: true
            font.pointSize: 20
            width: 50
        }
    }

    Rectangle {
        anchors.fill: parent
        color: CmnCfg.palette.darkGrey
    }

    onVisibilityChanged: {
        // 2 is the enum for Qwindow::Windowed
        // it is not in scope nor in the window namespace
        if (visibility === 2) {
            width = Math.min(image.sourceSize.width, 750)
            height = Math.min(image.sourceSize.height, 500)
            x = root.x
            y = root.y
        }
    }

    Flickable {
        id: flickable
        anchors.fill: parent
        ScrollBar.vertical: ScrollBar {}
        ScrollBar.horizontal: ScrollBar {}
        contentHeight: height
        contentWidth: width
        Image {
            id: image
            source: sourceAtc !== null ? "file:" + sourceAtc.attachmentPath(
                                             index) : ""
            fillMode: Image.PreserveAspectFit
            anchors.fill: parent
            mipmap: true
        }
    }

    PinchArea {
        id: pinchArea
        anchors.fill: parent
        onPinchUpdated: {
            imageWindow.scale += (pinch.scale - pinch.previousScale) / 2.0
            flickable.resizeContent(imageWindow.width * imageWindow.scale,
                                    imageWindow.height * imageWindow.scale,
                                    pinch.center)
        }
    }
}
