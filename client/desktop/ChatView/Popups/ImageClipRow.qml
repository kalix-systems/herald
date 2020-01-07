import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtGraphicalEffects 1.0
import QtQuick.Controls 2.5

//this component only exists for use in the galleryview, and is factored out
//for convenience. it relies on dynamic scoping, do not use elsewhere
ListView {
    id: clipScroll
    width: parent.width
    height: 80
    clip: true
    orientation: Qt.Horizontal
    anchors.top: flickable.bottom
    anchors.topMargin: CmnCfg.smallMargin
    anchors.horizontalCenter: flickable.horizontalCenter
    model: imageAttachments
    ScrollBar.horizontal: ScrollBar {}
    spacing: CmnCfg.smallMargin
    delegate: Rectangle {
        property var imageModel: model
        height: 64
        width: 64
        clip: true
        color: "transparent"
        property var imageSource: "file:" + imageAttachments[index].path
        Image {
            id: clip
            property var dims: JSON.parse(Herald.utils.imageScaling(
                                              imageAttachments[index].path, 64))
            sourceSize.width: dims.width
            sourceSize.height: dims.height
            anchors.centerIn: parent
            source: parent.imageSource
            ColorOverlay {
                id: overlay
                anchors.fill: parent
                source: parent
                visible: galleryPopup.currentIndex !== index
                color: CmnCfg.palette.black
                opacity: 0.7
                smooth: true
            }
        }
        MouseArea {
            anchors.fill: parent
            onClicked: {
                galleryPopup.currentIndex = index
                imageScale = 1.0
                flickable.contentHeight = flickable.height
                flickable.contentWidth = flickable.width
            }
            z: galleryPopup.z + 1
        }
    }
}
