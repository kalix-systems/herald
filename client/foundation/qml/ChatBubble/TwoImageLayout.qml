import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Row {
    height: 200
    spacing: CmnCfg.smallMargin
    property var firstImage
    property var secondImage
    property var imageTappedCallback: function () {
        throw "undefined callback"
    }

    Rectangle {
        height: 200
        width: height
        clip: true
        color: "transparent"
        Image {
            property var aspectRatio: firstImage.width / firstImage.height
            source: "file:" + firstImage.path
            height: aspectRatio > 1 ? 210 : 210 / aspectRatio
            width: aspectRatio > 1 ? 210 * aspectRatio : 210
            anchors.centerIn: parent
            fillMode: Image.PreserveAspectFit
            asynchronous: true
            MouseArea {
                onClicked: imageTappedCallBack
                anchors.fill: parent
            }
        }
    }

    Rectangle {
        height: 200
        width: height
        clip: true
        color: "transparent"
        Image {
            property var aspectRatio: secondImage.width / secondImage.height
            source: "file:" + secondImage.path
            height: aspectRatio > 1 ? 210 : 210 / aspectRatio
            width: aspectRatio > 1 ? 210 * aspectRatio : 210
            fillMode: Image.PreserveAspectFit
            anchors.centerIn: parent
            asynchronous: true
            MouseArea {
                onClicked: imageTappedCallBack
                anchors.fill: parent
            }
        }
    }
}
