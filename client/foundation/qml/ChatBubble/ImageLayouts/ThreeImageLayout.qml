import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

Row {
    id: wrapperRow
    height: CmnCfg.attachmentSize / 2
    spacing: CmnCfg.smallMargin
    property var firstImage
    property var secondImage
    property var thirdImage
    property var imageClickedCallBack: function () {
        throw "undefined callback"
    }

    Rectangle {
        id: wrapperRect
        height: CmnCfg.attachmentSize / 2
        width: height
        clip: true
        color: "transparent"
        Image {
            id: image1

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
                onClicked: imageClickedCallBack(image1.source)
                onPressAndHold: imageLongPressedCallBack(firstImage.path)
                anchors.fill: parent
            }
        }
    }

    Column {
        width: CmnCfg.attachmentSize / 2

        Rectangle {
            height: CmnCfg.attachmentSize / 4 - CmnCfg.smallMargin / 2
            width: CmnCfg.attachmentSize / 2
            clip: true

            Image {
                id: image2
                property var dims: JSON.parse(Herald.utils.imageScaling(
                                                  secondImage.path,
                                                  wrapperRow.height))
                source: "file:" + secondImage.path
                sourceSize.height: dims.height
                sourceSize.width: dims.width
                anchors.centerIn: parent
                fillMode: Image.PreserveAspectFit
                mipmap: false
                asynchronous: true
                MouseArea {
                    onClicked: imageClickedCallBack(image2.source)
                    onPressAndHold: imageLongPressedCallBack(secondImage.path)
                    anchors.fill: parent
                }
            }
        }
        spacing: CmnCfg.smallMargin

        Rectangle {
            height: CmnCfg.attachmentSize / 4 - CmnCfg.smallMargin / 2
            width: CmnCfg.attachmentSize / 2
            clip: true
            color: "transparent"
            Image {
                id: image3
                property var dims: JSON.parse(Herald.utils.imageScaling(
                                                  thirdImage.path,
                                                  wrapperRow.height))
                source: "file:" + thirdImage.path
                sourceSize.height: dims.height
                sourceSize.width: dims.width
                fillMode: Image.PreserveAspectFit
                anchors.centerIn: parent
                mipmap: false
                asynchronous: true
                MouseArea {
                    onClicked: imageClickedCallBack(image3.source)
                    onPressAndHold: imageLongPressedCallBack(thirdImage.path)
                    anchors.fill: parent
                }
            }
        }
    }
}
