import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.0

Row {
    property var firstImage
    property var secondImage
    property var thirdImage
    property var fourthImage
    property int count
    property var imageClickedCallBack: function () {
        throw "undefined callback"
    }
    onPositioningComplete: bubbleRoot.attachmentsLoaded()
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
            height: aspectRatio > 1 ? 150 : 150 / aspectRatio
            width: aspectRatio > 1 ? 150 * aspectRatio : 150
            anchors.centerIn: parent
            mipmap: false
            asynchronous: true
            MouseArea {
                onClicked: imageClickedCallBack(parent.source)
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
                height: aspectRatio > 1 ? 150 : 150 / aspectRatio
                width: aspectRatio > 1 ? 150 * aspectRatio : 150
                anchors.centerIn: parent
                mipmap: false
                asynchronous: true
                MouseArea {
                    onClicked: imageClickedCallBack(parent.source)
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
                    height: aspectRatio > 1 ? 75 : 75 / aspectRatio
                    width: aspectRatio > 1 ? 75 * aspectRatio : 75
                    anchors.centerIn: parent
                    mipmap: false
                    asynchronous: true
                    MouseArea {
                        onClicked: imageClickedCallBack(parent.source)
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
                    height: aspectRatio > 1 ? 75 : 75 / aspectRatio
                    width: aspectRatio > 1 ? 75 * aspectRatio : 75
                    anchors.centerIn: parent
                    mipmap: false
                    asynchronous: true
                    MouseArea {
                        onClicked: imageClickedCallBack(parent.source)
                        anchors.fill: parent
                    }
                }

                ColorOverlay {
                    anchors.fill: parent
                    source: parent
                    color: CmnCfg.palette.black
                    opacity: 0.5
                }

                Text {
                    anchors.centerIn: parent
                    text: "+ " + count
                    color: CmnCfg.palette.white
                    font.family: CmnCfg.chatFont.name
                    font.weight: Font.DemiBold
                    font.pointSize: 20
                }
            }
        }
    }
}
