import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Image {
    property var firstImage
    property var aspectRatio: firstImage.width / firstImage.height
    sourceSize.width: aspectRatio < 1 ? 300 * aspectRatio : 300
    sourceSize.height: aspectRatio < 1 ? 300 : 300 / aspectRatio
    source: "file:" + firstImage.path
    fillMode: Image.PreserveAspectCrop
    asynchronous: true
}
