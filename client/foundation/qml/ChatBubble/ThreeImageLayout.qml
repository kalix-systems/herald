import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

Row {
    id: wrapperRow
    height: 150
    spacing: CmnCfg.smallMargin
    property var firstImage
    property var secondImage
    property var thirdImage
    property var imageClickedCallBack: function () {
        throw "undefined callback"
    }

    onPositioningComplete: bubbleRoot.attachmentsLoaded()
    Rectangle {
        id: wrapperRect
        height: 150
        width: height
        clip: true
        color: "transparent"
        Image {
            id: image1
            property var aspectRatio: firstImage.width / firstImage.height
            source: "file:" + firstImage.path
            sourceSize.height: aspectRatio > 1 ? 150 : 150 / aspectRatio
            sourceSize.width: aspectRatio > 1 ? 150 * aspectRatio : 150
            height: aspectRatio > 1 ? 150 : 150 / aspectRatio
            width: aspectRatio > 1 ? 150 * aspectRatio : 150
            anchors.centerIn: parent
            fillMode: Image.PreserveAspectFit
            mipmap: false
            asynchronous: true
            MouseArea {
                onClicked: imageClickedCallBack(image1.source)
                anchors.fill: parent
            }
        }
    }

    Column {
        width: 150

        Rectangle {
            height: 75 - CmnCfg.smallMargin / 2
            width: 150
            clip: true

            Image {
                id: image2
                property var aspectRatio: secondImage.width / secondImage.height
                source: "file:" + secondImage.path
                sourceSize.height: aspectRatio > 1 ? 150 : 150 / aspectRatio
                sourceSize.width: aspectRatio > 1 ? 150 * aspectRatio : 150
                height: aspectRatio > 1 ? 150 : 150 / aspectRatio
                width: aspectRatio > 1 ? 150 * aspectRatio : 150
                anchors.centerIn: parent
                fillMode: Image.PreserveAspectFit
                mipmap: false
                asynchronous: true
                MouseArea {
                    onClicked: imageClickedCallBack(image2.source)
                    anchors.fill: parent
                }
            }
        }
        spacing: CmnCfg.smallMargin

        Rectangle {
            height: 75 - CmnCfg.smallMargin / 2
            width: 150
            clip: true
            color: "transparent"
            Image {
                id: image3
                property var aspectRatio: thirdImage.width / thirdImage.height
                source: "file:" + thirdImage.path
                sourceSize.height: aspectRatio > 1 ? 150 : 150 / aspectRatio
                sourceSize.width: aspectRatio > 1 ? 150 * aspectRatio : 150
                height: aspectRatio > 1 ? 150 : 150 / aspectRatio
                width: aspectRatio > 1 ? 150 * aspectRatio : 150
                fillMode: Image.PreserveAspectFit
                anchors.centerIn: parent
                mipmap: false
                asynchronous: true
                MouseArea {
                    onClicked: imageClickedCallBack(image3.source)
                    anchors.fill: parent
                }
            }
        }
    }
}
