import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick.Window 2.13

Window {
    property Attachments sourceAtc

    height: image.sourceSize.height
    width: image.sourceSize.width
    title: sourceAtc !== null ? sourceAtc.attachmentPath(0).substring(
                                    sourceAtc.attachmentPath(0).lastIndexOf(
                                        '/') + 1) : ""
    Connections {
        onShow: {
            y = root.y
            x = root.width + root.x
        }
    }

    Flickable {
        anchors.fill: parent
        Image {
            id: image
            source: sourceAtc !== null ? "file:" + sourceAtc.attachmentPath(
                                             0) : ""
        }
    }
}
