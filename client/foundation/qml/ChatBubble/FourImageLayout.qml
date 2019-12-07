import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Row {
    property var firstImage
    property var secondImage
    property var thirdImage
    property var fourthImage
    property var imageTappedCallback: function () {
        throw "undefined callback"
    }

    height: 150
    spacing: CmnCfg.smallMargin
    Rectangle {
        height: 150
        width: height
        clip: true
        color: "transparent"
        Image {
            property var aspectRatio: firstImage.width / firstImage.height
            source: "file:" + firstImage.path
            sourceSize.height: aspectRatio > 1 ? 150 : 150 / aspectRatio
            sourceSize.width: aspectRatio > 1 ? 150 * aspectRatio : 150
            height: aspectRatio > 1 ? 150 : 150 / aspectRatio
            width: aspectRatio > 1 ? 150 * aspectRatio : 150
            anchors.centerIn: parent
            mipmap: false
            asynchronous: true
            MouseArea {
                onClicked: imageTappedCallBack(parent.source)
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
            color: "transparent"
            Image {
                property var aspectRatio: secondImage.width / secondImage.height
                source: "file:" + secondImage.path
                sourceSize.height: aspectRatio > 1 ? 150 : 150 / aspectRatio
                sourceSize.width: aspectRatio > 1 ? 150 * aspectRatio : 150
                height: aspectRatio > 1 ? 150 : 150 / aspectRatio
                width: aspectRatio > 1 ? 150 * aspectRatio : 150
                anchors.centerIn: parent
                mipmap: false
                asynchronous: true
                MouseArea {
                    onClicked: imageTappedCallBack(parent.source)
                    anchors.fill: parent
                }
            }
        }
        spacing: CmnCfg.smallMargin

        Row {
            height: 75
            spacing: CmnCfg.smallMargin
            Rectangle {
                height: 75 - CmnCfg.smallMargin / 2
                width: height
                clip: true
                color: "transparent"
                Image {
                    property var aspectRatio: thirdImage.width / thirdImage.height
                    source: "file:" + thirdImage.path
                    sourceSize.height: aspectRatio > 1 ? 150 : 150 / aspectRatio
                    sourceSize.width: aspectRatio > 1 ? 150 * aspectRatio : 150
                    height: aspectRatio > 1 ? 75 : 75 / aspectRatio
                    width: aspectRatio > 1 ? 75 * aspectRatio : 75
                    anchors.centerIn: parent
                    mipmap: false
                    asynchronous: true
                    MouseArea {
                        onClicked: imageTappedCallBack(parent.source)
                        anchors.fill: parent
                    }
                }
            }

            Rectangle {
                height: 75 - CmnCfg.smallMargin / 2
                width: height
                clip: true
                color: "transparent"
                Image {
                    property var aspectRatio: fourthImage.width / fourthImage.height
                    source: "file:" + fourthImage.path
                    sourceSize.height: aspectRatio > 1 ? 75 : 75 / aspectRatio
                    sourceSize.width: aspectRatio > 1 ? 75 * aspectRatio : 75
                    height: aspectRatio > 1 ? 75 : 75 / aspectRatio
                    width: aspectRatio > 1 ? 75 * aspectRatio : 75
                    anchors.centerIn: parent
                    mipmap: false
                    asynchronous: true
                    MouseArea {
                        onClicked: imageTappedCallBack(parent.source)
                        anchors.fill: parent
                    }
                }
            }
        }
    }
}
