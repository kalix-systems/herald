import QtQuick 2.14
import QtQuick.Controls 2.14
import Qt.labs.platform 1.1
import QtGraphicalEffects 1.12
import LibHerald 1.0
import "./../"
import "../js/utils.mjs" as Utils

// A visual list of documents
ListView {
    id: fileList
    interactive: false
    width: contentItem.childrenRect.width + CmnCfg.smallMargin * 2
    height: 24 * (!expand ? Math.min(docParsed.length, 4) : docParsed.length)
    spacing: CmnCfg.microMargin
    clip: true

    boundsBehavior: Flickable.StopAtBounds

    delegate: Row {
        spacing: CmnCfg.smallMargin / 2
        visible: wrapperCol.expand ? true : index < 4

        IconButton {
            id: downloadIcon
            icon.source: "qrc:/file-download-icon.svg"
            icon.height: 20
            icon.width: height
            onClicked: {
                attachmentDownloader.filePath = path
                attachmentDownloader.open()
            }

            mouseArea.onEntered: bubbleActual.hoverHighlight = true
            mouseArea.onExited: if (!bubbleActual.hitbox.containsMouse) {
                                    bubbleActual.hoverHighlight = false
                                }
        }

        Text {
            id: fileName
            color: CmnCfg.palette.black
            text: fileNameMetrics.elidedText
            font.family: CmnCfg.chatFont.name
            font.pixelSize: 13
            font.weight: Font.Medium

            TextMetrics {
                id: fileNameMetrics
                text: name
                font.family: CmnCfg.chatFont.name
                font.pixelSize: 13
                font.weight: Font.Medium
                elide: Text.ElideMiddle
                elideWidth: bubbleRoot.maxWidth * 0.8 - fileSize.width
                            + downloadIcon.width + CmnCfg.smallMargin * 2
            }
        }

        Text {
            id: fileSize
            text: Utils.friendlyFileSize(size)
            font.family: CmnCfg.chatFont.name
            font.pixelSize: 10
            font.weight: Font.Light
            color: CmnCfg.palette.darkGrey
            anchors.verticalCenter: fileName.verticalCenter
        }
    }
}
