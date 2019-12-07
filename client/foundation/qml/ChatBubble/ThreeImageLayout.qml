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
            height: aspectRatio > 1 ? 150 : 150 / aspectRatio
            width: aspectRatio > 1 ? 150 * aspectRatio : 150
            anchors.centerIn: parent
            fillMode: Image.PreserveAspectFit
            mipmap: true
            asynchronous: true
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
                height: aspectRatio > 1 ? 150 : 150 / aspectRatio
                width: aspectRatio > 1 ? 150 * aspectRatio : 150
                anchors.centerIn: parent
                fillMode: Image.PreserveAspectFit
                mipmap: true
                asynchronous: true
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
                height: aspectRatio > 1 ? 150 : 150 / aspectRatio
                width: aspectRatio > 1 ? 150 * aspectRatio : 150
                fillMode: Image.PreserveAspectFit
                anchors.centerIn: parent
                mipmap: true
                asynchronous: true
            }
        }
    }
}
