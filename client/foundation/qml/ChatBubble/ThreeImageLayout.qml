import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

Row {
    id: wrapperRow
    height: 200
    spacing: CmnCfg.smallMargin
    property var firstImage
    property var secondImage
    property var thirdImage
    property var imageTappedCallback: function () {
        throw "undefined callback"
    }

    Rectangle {
        id: wrapperRect
        height: 200
        width: height
        clip: true
        color: "transparent"
        Image {
            id: image1
            property var aspectRatio: firstImage.width / firstImage.height
            source: "file:" + firstImage.path
            height: aspectRatio > 1 ? 210 : 210 / aspectRatio
            width: aspectRatio > 1 ? 210 * aspectRatio : 210
            anchors.centerIn: parent
            fillMode: Image.PreserveAspectFit
            mipmap: true
            asynchronous: true
            MouseArea {
                onClicked: imageTappedCallBack(image1.source)
                anchors.fill: parent
            }
        }
    }

    Column {
        width: 200

        Rectangle {
            height: 100 - CmnCfg.smallMargin / 2
            width: 200
            clip: true

            Image {
                id: image2
                property var aspectRatio: secondImage.width / secondImage.height
                source: "file:" + secondImage.path
                height: aspectRatio > 1 ? 210 : 210 / aspectRatio
                width: aspectRatio > 1 ? 210 * aspectRatio : 210
                anchors.centerIn: parent
                fillMode: Image.PreserveAspectFit
                mipmap: true
                asynchronous: true
                MouseArea {
                    onClicked: imageTappedCallBack(image2.source)
                    anchors.fill: parent
                }
            }
        }
        spacing: CmnCfg.smallMargin

        Rectangle {
            height: 100 - CmnCfg.smallMargin / 2
            width: 200
            clip: true
            color: "transparent"
            Image {
                id: image3
                property var aspectRatio: thirdImage.width / thirdImage.height
                source: "file:" + thirdImage.path
                height: aspectRatio > 1 ? 210 : 210 / aspectRatio
                width: aspectRatio > 1 ? 210 * aspectRatio : 210
                fillMode: Image.PreserveAspectFit
                anchors.centerIn: parent
                mipmap: true
                asynchronous: true
                MouseArea {
                    onClicked: imageTappedCallBack(image3.source)
                    anchors.fill: parent
                }
            }
        }
    }
}
