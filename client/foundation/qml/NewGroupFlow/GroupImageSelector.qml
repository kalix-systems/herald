import QtQuick 2.12
import QtQuick.Dialogs 1.3
import QtGraphicalEffects 1.0
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "qrc:/imports/"

Rectangle {
    id: imageSelector
    property alias imageSource: groupImageLoader.imageSource
    property color backgroundColor
    width: CmnCfg.avatarSize
    height: width

    Loader {
        id: groupImageLoader
        active: false
        anchors.fill: parent
        z: 100
        property string imageSource

        sourceComponent: Image {
            source: imageSource
            anchors.fill: parent
            fillMode: Image.PreserveAspectCrop
            asynchronous: true

            ColorOverlay {
                id: overlay
                anchors.fill: parent
                source: parent
                visible: imageHover.containsMouse
                         && groupImageLoader.imageSource !== ""
                color: CmnCfg.palette.black
                opacity: 0.5
                smooth: true
            }

            IconButton {
                id: clearPhoto
                source: "qrc:/x-icon.svg"
                anchors.centerIn: parent
                visible: imageHover.containsMouse
                         && groupImageLoader.imageSource !== ""
                onClicked: groupImageLoader.imageSource = ""
                fill: CmnCfg.palette.white
                opacity: 1.0
                hoverEnabled: true
            }
        }

        MouseArea {
            anchors.fill: parent
            hoverEnabled: true
            id: imageHover
            onClicked: mouse.accepted = false
            onReleased: mouse.accepted = false
            onPressed: mouse.accepted = false
            cursorShape: Qt.PointingHandCursor
        }
    }

    IconButton {
        anchors.centerIn: parent
        source: "qrc:/camera-icon.svg"
        fill: CmnCfg.palette.offBlack
        onClicked: groupPicDialogue.open()
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
