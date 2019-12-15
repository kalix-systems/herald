import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12
import "./../"
import Qt.labs.platform 1.1
//import QtQuick.Dialogs 1.3
import "../js/utils.mjs" as Utils

ListView {
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

    delegate: Item {
        id: fileRow
        width: bubbleRoot.width
        height: 20
        clip: true

        RowLayout {

            anchors.left: parent.left
            anchors.verticalCenter: parent.verticalCenter
            anchors.leftMargin: CmnCfg.smallMargin

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
                Layout.maximumWidth: bubbleRoot.maxWidth - fileSize.width - 40
                                     - CmnCfg.smallMargin * 2
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

        ButtonForm {
            id: downloadIcon
            anchors.verticalCenter: parent.verticalCenter
            anchors.right: parent.right
            source: "qrc:/download-icon.svg"
            icon.width: 20
            icon.height: 20
            fill: CmnCfg.palette.black
            onClicked: fileChooser.open()
        }

        FileDialog {
            id: fileChooser
            folder: StandardPaths.writableLocation(
                        StandardPaths.DesktopLocation)
            onAccepted: Herald.utils.saveFile(path, fileUrl)
        }
    }
}
