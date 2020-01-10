import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

// TODO: All these image layouts should be in an image layout subdirectory
Row {
    height: 150
    spacing: CmnCfg.smallMargin
    property var firstImage
    property var secondImage
    property var imageClickedCallBack: function () {
        throw "undefined callback"
    }
    Rectangle {
        height: 150
        width: height
        clip: true
        color: "transparent"
        property var dims: JSON.parse(Herald.utils.imageScaling(
                                          firstImage.path, 150))
        Image {
            source: "file:" + firstImage.path
            sourceSize.height: parent.dims.height
            sourceSize.width: parent.dims.width
            anchors.centerIn: parent
            fillMode: Image.PreserveAspectFit
            asynchronous: true
            MouseArea {
                onClicked: imageClickedCallBack(parent.source)
                anchors.fill: parent
            }
        }
    }

    Rectangle {
        height: 150
        width: height
        clip: true
        color: "transparent"
        property var dims: JSON.parse(Herald.utils.imageScaling(
                                          secondImage.path, 150))
        Image {
            source: "file:" + secondImage.path
            sourceSize.height: parent.dims.height
            sourceSize.width: parent.dims.width
            fillMode: Image.PreserveAspectFit
            anchors.centerIn: parent
            asynchronous: true
            MouseArea {
                onClicked: imageClickedCallBack(parent.source)
                anchors.fill: parent
            }
        }
    }
}