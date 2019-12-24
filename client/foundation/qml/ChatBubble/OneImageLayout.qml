import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Row {
    id: wrapperRow

    height: image.height
    property var firstImage
    property var aspectRatio: firstImage.width / firstImage.height
    property var imageClickedCallBack: function () {
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
            onClicked: wrapperRow.imageClickedCallBack(image.source)
            anchors.fill: parent
        }
    }
}
