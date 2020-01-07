import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Row {
    property var firstImage
    property var secondImage
    property var thirdImage
    property var fourthImage
    property var imageClickedCallBack: function () {
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
            property var dims: JSON.parse(Herald.utils.imageScaling(
                                              firstImage.path, parent.height))
            source: "file:" + firstImage.path
            sourceSize.height: dims.height
            sourceSize.width: dims.width
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
                property var dims: JSON.parse(Herald.utils.imageScaling(
                                                  secondImage.path, 150))
                source: "file:" + secondImage.path
                sourceSize.height: dims.height
                sourceSize.width: dims.width
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
                    property var dims: JSON.parse(Herald.utils.imageScaling(
                                                      thirdImage.path, 75))
                    source: "file:" + thirdImage.path
                    sourceSize.height: dims.height
                    sourceSize.width: dims.width
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
                    property var dims: JSON.parse(Herald.utils.imageScaling(
                                                      fourthImage.path, 75))

                    source: "file:" + fourthImage.path
                    sourceSize.height: dims.height
                    sourceSize.width: dims.width
                    anchors.centerIn: parent
                    mipmap: false
                    asynchronous: true
                    MouseArea {
                        onClicked: imageClickedCallBack(parent.source)
                        anchors.fill: parent
                    }
                }
            }
        }
    }
}
