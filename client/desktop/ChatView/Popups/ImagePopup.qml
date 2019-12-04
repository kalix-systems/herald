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
    Rectangle {
        anchors.fill: parent
        color: CmnCfg.palette.medGrey
    }

    Flickable {
        anchors.fill: parent
        Image {
            id: image
            source: sourceAtc !== null ? "file:" + sourceAtc.attachmentPath(
                                             0) : ""
            fillMode: Image.PreserveAspectFit
            anchors.fill: parent

            PinchArea {
                id: pinch
                anchors.fill: parent
            }
        }
        ScrollBar.vertical: ScrollBar {}
        ScrollBar.horizontal: ScrollBar {}
    }
}
