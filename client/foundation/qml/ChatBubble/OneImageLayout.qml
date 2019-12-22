import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Row {
    height: image.height
    id: wrapperRow
    property var firstImage
    property var aspectRatio: firstImage.width / firstImage.height
    property var imageTappedCallback: function () {
        throw "undefined callback"
    }
    onPositioningComplete: bubbleRoot.attachmentsLoaded()

    Image {
        id: image
        sourceSize.width: aspectRatio < 1 ? 300 * aspectRatio : 300
        sourceSize.height: aspectRatio < 1 ? 300 : 300 / aspectRatio
        source: "file:" + firstImage.path
        fillMode: Image.PreserveAspectCrop
        asynchronous: true
        MouseArea {
            onClicked: imageClickedCallBack(wrapperRow.source)
            anchors.fill: parent
        }
    }
}
