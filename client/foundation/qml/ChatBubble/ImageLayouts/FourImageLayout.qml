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
    height: CmnCfg.attachmentSize / 2
    spacing: CmnCfg.smallMargin
    Rectangle {
        height: CmnCfg.attachmentSize / 2
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
            fillMode: Image.PreserveAspectFit
            mipmap: false
            asynchronous: true
            MouseArea {
                onClicked: imageClickedCallBack(parent.source)
                onPressAndHold: imageLongPressedCallBack(firstImage.path)
                anchors.fill: parent
            cursorShape: Qt.PointingHandCursor
            }
        }
    }
    Column {
        width: CmnCfg.attachmentSize / 2

        Rectangle {
            height: CmnCfg.attachmentSize / 4 - CmnCfg.smallMargin / 2
            width: CmnCfg.attachmentSize / 2
            clip: true
            color: "transparent"
            Image {
                property var dims: JSON.parse(Herald.utils.imageScaling(
                                                  secondImage.path,
                                                  CmnCfg.attachmentSize / 2))
                source: "file:" + secondImage.path
                sourceSize.height: dims.height
                sourceSize.width: dims.width
                anchors.centerIn: parent
                fillMode: Image.PreserveAspectFit
                mipmap: false
                asynchronous: true
                MouseArea {
                    onClicked: imageClickedCallBack(parent.source)
                    onPressAndHold: imageLongPressedCallBack(secondImage.path)
                    anchors.fill: parent
            cursorShape: Qt.PointingHandCursor
                }
            }
        }
        spacing: CmnCfg.smallMargin

        Row {
            height: CmnCfg.attachmentSize / 4
            spacing: CmnCfg.smallMargin
            Rectangle {
                height: parent.height - CmnCfg.smallMargin / 2
                width: height
                clip: true
                color: "transparent"
                Image {
                    property var dims: JSON.parse(
                                           Herald.utils.imageScaling(
                                               thirdImage.path,
                                               CmnCfg.attachmentSize / 4))
                    source: "file:" + thirdImage.path
                    sourceSize.height: dims.height
                    sourceSize.width: dims.width
                    anchors.centerIn: parent
                    mipmap: false
                    asynchronous: true
                    fillMode: Image.PreserveAspectFit
                    MouseArea {
                        onClicked: imageClickedCallBack(parent.source)
                        onPressAndHold: imageLongPressedCallBack(
                                            thirdImage.path)
                        anchors.fill: parent
            cursorShape: Qt.PointingHandCursor
                    }
                }
            }

            Rectangle {
                height: parent.height - CmnCfg.smallMargin / 2
                width: height
                clip: true
                color: "transparent"
                Image {
                    property var dims: JSON.parse(
                                           Herald.utils.imageScaling(
                                               fourthImage.path,
                                               CmnCfg.attachmentSize / 4))

                    source: "file:" + fourthImage.path
                    sourceSize.height: dims.height
                    sourceSize.width: dims.width
                    anchors.centerIn: parent
                    mipmap: false
                    asynchronous: true
                    fillMode: Image.PreserveAspectFit
                    MouseArea {
                        onClicked: imageClickedCallBack(parent.source)
                        onPressAndHold: imageLongPressedCallBack(
                                            fourthImage.path)
                        anchors.fill: parent
            cursorShape: Qt.PointingHandCursor
                    }
                }
            }
        }
    }
}
