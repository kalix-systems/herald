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

    height: 200
    spacing: CmnCfg.smallMargin
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
            mipmap: true
            asynchronous: true
            MouseArea {
                onClicked: imageTappedCallBack
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
            color: "transparent"
            Image {
                property var aspectRatio: secondImage.width / secondImage.height
                source: "file:" + secondImage.path
                height: aspectRatio > 1 ? 210 : 210 / aspectRatio
                width: aspectRatio > 1 ? 210 * aspectRatio : 210
                anchors.centerIn: parent
                mipmap: true
                asynchronous: true
                MouseArea {
                    onClicked: imageTappedCallBack
                    anchors.fill: parent
                }
            }
        }
        spacing: CmnCfg.smallMargin

        Row {
            height: 100
            spacing: CmnCfg.smallMargin
            Rectangle {
                height: 100 - CmnCfg.smallMargin / 2
                width: height
                clip: true
                color: "transparent"
                Image {
                    property var aspectRatio: thirdImage.width / thirdImage.height
                    source: "file:" + thirdImage.path
                    height: aspectRatio > 1 ? 100 : 100 / aspectRatio
                    width: aspectRatio > 1 ? 100 * aspectRatio : 100
                    anchors.centerIn: parent
                    mipmap: true
                    asynchronous: true
                    MouseArea {
                        onClicked: imageTappedCallBack
                        anchors.fill: parent
                    }
                }
            }

            Rectangle {
                height: 100 - CmnCfg.smallMargin / 2
                width: height
                clip: true
                color: "transparent"
                Image {
                    property var aspectRatio: fourthImage.width / fourthImage.height
                    source: "file:" + fourthImage.path
                    height: aspectRatio > 1 ? 100 : 100 / aspectRatio
                    width: aspectRatio > 1 ? 100 * aspectRatio : 100
                    anchors.centerIn: parent
                    mipmap: true
                    asynchronous: true
                    MouseArea {
                        onClicked: imageTappedCallBack
                        anchors.fill: parent
                    }
                }
            }
        }
    }
}
