import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import QtQuick.Dialogs 1.3
import QtMultimedia 5.13

Rectangle  {
    id: topRect
    anchors.top: parent.top
    height: 70
    width: parent.width
   color: CmnCfg.palette.paneColor
   property alias profPic: groupImageLoader.imageSource

    Rectangle {
        anchors.top: parent.top
        anchors.topMargin: CmnCfg.largeMargin
        anchors.horizontalCenter: parent.horizontalCenter
        width: 42
        height: width
        color: "black"

        Loader {
            id: groupImageLoader
            active: false
            z: 100
            property string imageSource
            anchors.fill: parent
            sourceComponent: Image {
                source: imageSource
                anchors.fill: parent
                fillMode: Image.PreserveAspectCrop
            }
        }

        Common.ButtonForm {
            anchors.centerIn: parent
            source: "qrc:/camera-icon.svg"
            fill: CmnCfg.palette.paneColor
            onClicked: groupPicDialogue.open()
        }
    }

    FileDialog {
        id: groupPicDialogue
        folder: shortcuts.home
        nameFilters: ["Image File (*.jpg *.png *.jpeg)"]
        selectedNameFilter: "Image File"

        onSelectionAccepted: {
            groupImageLoader.active = true
            groupImageLoader.imageSource = fileUrl
        }
    }
}
