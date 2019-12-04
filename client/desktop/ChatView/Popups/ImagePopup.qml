import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick.Window 2.13

Window {
    id: imageWindow
    property real scale: 1.0
    property bool freeScroll: scale === 1.0
    property Attachments sourceAtc
    title: sourceAtc !== null ? sourceAtc.attachmentPath(0).substring(
                                    sourceAtc.attachmentPath(0).lastIndexOf(
                                        '/') + 1) : ""

    width: Math.min(image.sourceSize.width, 750)
    height: Math.min(image.sourceSize.height, 500)
    minimumWidth: 350
    minimumHeight: 150

    Item {
        id: controls
    }

    Rectangle {
        id: background
        anchors.fill: parent
        color: CmnCfg.palette.medGrey
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
                                             0) : ""
            fillMode: Image.PreserveAspectFit
            anchors.fill: parent
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
