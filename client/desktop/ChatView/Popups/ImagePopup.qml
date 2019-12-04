import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick.Window 2.13

Window {
    id: imageWindow
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
            image.scale = 1
            x = root.x
            y = root.y
        }
    }

    Flickable {
        id: flickable
        // allow for scrolling
        anchors.centerIn: background
        anchors.fill: parent
        Image {
            id: image
            source: sourceAtc !== null ? "file:" + sourceAtc.attachmentPath(
                                             0) : ""
            fillMode: Image.PreserveAspectFit
            width: Math.min(image.sourceSize.width, 750)
            anchors.centerIn: parent
            anchors.fill: parent
        }
        PinchArea {
            id: pinchArea
            property real imageScale: 1.0
            anchors.fill: parent
            onPinchUpdated: {

            }
        }
        ScrollBar.vertical: ScrollBar {}
        ScrollBar.horizontal: ScrollBar {}
    }
}
