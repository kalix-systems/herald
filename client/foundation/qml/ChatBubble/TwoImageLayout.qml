import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Row {
    height: 150
    spacing: CmnCfg.smallMargin
    property var firstImage
    property var secondImage
    Rectangle {
        height: 150
        width: height
        clip: true
        color: "transparent"
        Image {
            property var aspectRatio: firstImage.width / firstImage.height
            source: "file:" + firstImage.path
            height: aspectRatio > 1 ? 150 : 150 / aspectRatio
            width: aspectRatio > 1 ? 150 * aspectRatio : 150
            anchors.centerIn: parent
            fillMode: Image.PreserveAspectFit
            asynchronous: true
        }
    }

    Rectangle {
        height: 150
        width: height
        clip: true
        color: "transparent"
        Image {
            property var aspectRatio: secondImage.width / secondImage.height
            source: "file:" + secondImage.path
            height: aspectRatio > 1 ? 150 : 150 / aspectRatio
            width: aspectRatio > 1 ? 150 * aspectRatio : 150
            fillMode: Image.PreserveAspectFit
            anchors.centerIn: parent
            asynchronous: true
        }
    }
}
