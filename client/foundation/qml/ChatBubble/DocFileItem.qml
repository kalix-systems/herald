import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12
import "./../"
import Qt.labs.platform 1.1
//import QtQuick.Dialogs 1.3
import "../js/utils.mjs" as Utils

Item {
    property alias fileModel: docFileItemRoot.model
    property alias downloadModel: downloadList.model
    height: childrenRect.height
    width: childrenRect.width

    Rectangle {
        anchors.fill: parent
        color: "red"
        opacity: 0.5
    }

    ListView {
        anchors.left: parent.left
        anchors.top: parent.top
        id: docFileItemRoot
        interactive: false
        width: contentItem.childrenRect.width
        height: contentItem.childrenRect.height
        spacing: CmnCfg.smallMargin / 2

        Rectangle {
            anchors.fill: parent
            border.color: "black"
            border.width: 1
            opacity: 0
        }

        delegate: RowLayout {
            clip: true
            Image {
                id: fileIcon
                source: "qrc:/file-icon.svg"
                height: 20
                width: height
            }
            Text {
                id: fileName
                color: CmnCfg.palette.black
                text: name
                font.family: CmnCfg.chatFont.name
                font.pixelSize: 13
                font.weight: Font.Medium
                elide: Text.ElideMiddle
                Layout.maximumWidth: bubbleRoot.maxWidth - fileSize.width - 20
                                     - CmnCfg.smallMargin * 2 - downloadList.width
            }

            Text {
                id: fileSize
                text: Utils.friendlyFileSize(size)
                font.family: CmnCfg.chatFont.name
                font.pixelSize: 10
                font.weight: Font.Light
                color: CmnCfg.palette.darkGrey
            }
        }

        FileDialog {
            id: fileChooser
            folder: StandardPaths.writableLocation(
                        StandardPaths.DesktopLocation)
            onAccepted: Herald.utils.saveFile(path, fileUrl)
        }
    }

    ListView {
        id: downloadList
        height: contentItem.childrenRect.height
        width: contentItem.childrenRect.width
        anchors.right: parent.right
        anchors.top: parent.top
        spacing: CmnCfg.smallMargin / 2
        delegate: ButtonForm {
            id: downloadIcon
            //anchors.verticalCenter: parent.verticalCenter
            source: "qrc:/download-icon.svg"
            icon.width: 20
            icon.height: 20
            fill: CmnCfg.palette.black
            onClicked: fileChooser.open()
        }
    }
}
