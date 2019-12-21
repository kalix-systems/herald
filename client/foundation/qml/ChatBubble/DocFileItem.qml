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
    height: 24 * Math.min(docParsed.length, 5)
    spacing: CmnCfg.smallMargin / 2
    clip: true

    ScrollBar.vertical: ScrollBar {
        id: scrollBar
        policy: contentHeight > height ? ScrollBar.AlwaysOn : ScrollBar.AlwaysOff
    }
    boundsBehavior: Flickable.StopAtBounds

    delegate: Row {
        spacing: CmnCfg.smallMargin / 2
        ButtonForm {
            id: downloadIcon
            icon.source: "qrc:/file-download-icon.svg"
            icon.height: 20
            icon.width: height
            onClicked: {
                attachmentDownloader.filePath = path
                attachmentDownloader.open()
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
