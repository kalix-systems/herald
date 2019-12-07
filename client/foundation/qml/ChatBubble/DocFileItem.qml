import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12
import "./../"
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "../js/utils.mjs" as Utils

ListView {
    id: docFileItemRoot
    interactive: false
    width: 200
    delegate: Item {
        id: fileRow
        width: Math.max(bubbleRoot.width - CmnCfg.smallMargin * 2, 100)
        height: 24
        clip: true

        Image {
            anchors.left: parent.left
            id: fileIcon
            anchors.verticalCenter: parent.verticalCenter
            source: "qrc:/file-icon.svg"
            height: 20
            width: height
        }

        TextMetrics {
            id: nameMetrics
            text: name
            elide: Text.ElideMiddle
            elideWidth: fileRow.width * 0.7 - fileSize.width
        }

        Text {
            anchors.left: fileIcon.right
            anchors.leftMargin: CmnCfg.smallMargin
            anchors.verticalCenter: parent.verticalCenter
            id: fileName
            color: CmnCfg.palette.black
            text: nameMetrics.elidedText
            font.family: CmnCfg.chatFont.name
            font.pixelSize: 13
        }

        Text {
            id: fileSize
            anchors.left: fileName.right
            anchors.leftMargin: CmnCfg.smallMargin
            anchors.verticalCenter: parent.verticalCenter
            text: Utils.friendlyFileSize(size)
            font.family: CmnCfg.chatFont.name
            font.pixelSize: 10
            color: CmnCfg.palette.darkGrey
        }

        ButtonForm {
            id: downloadIcon
            anchors.verticalCenter: parent.verticalCenter
            anchors.right: parent.right
            source: "qrc:/download-icon.svg"
            height: 20
            width: height
            fill: CmnCfg.palette.black
            onClicked: {
                fileChooser.open()
            }
        }

        FileDialog {
            id: fileChooser
            selectExisting: false
            selectFolder: true
            selectMultiple: false
            folder: StandardPaths.writableLocation(
                        StandardPaths.DesktopLocation)
            onAccepted: {
                herald.utils.saveFile(path, fileUrl)
            }
        }
    }
}
