import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12
import "./../"
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "../js/utils.mjs" as Utils

Row {
    property alias fileModel: fileList.model
    property alias downloadModel: downloadList.model
    height: docParsed.length * 28 //childrenRect.height
    width: childrenRect.width
    spacing: CmnCfg.smallMargin / 2

    ListView {
        id: fileList
        anchors.top: parent.top
        interactive: false
        width: contentItem.childrenRect.width
        height: docParsed.length * 24 //fileModel.length * 20 //contentItem.childrenRect.height
        spacing: CmnCfg.smallMargin / 2

        delegate: RowLayout {
            clip: true

            // Constrain the maximum width of fileName to force elision when necessary
            readonly property real _constraint: fileSize.width + fileIcon.width
                                                + downloadList.width + CmnCfg.smallMargin * 2
            ButtonForm {
                id: fileIcon
                icon.source: "qrc:/file-icon.svg"
                icon.height: 20
                icon.width: height
            }

            Text {
                id: fileName
                color: CmnCfg.palette.black
                text: name
                font.family: CmnCfg.chatFont.name
                font.pixelSize: 13
                font.weight: Font.Medium
                elide: Text.ElideMiddle

                Layout.maximumWidth: bubbleRoot.maxWidth * 0.8 - parent._constraint
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
    }

    ListView {
        id: downloadList
        height: fileList.height
        width: contentItem.childrenRect.width
        anchors.top: parent.top
        spacing: CmnCfg.smallMargin / 2
        interactive: false
        delegate: ButtonForm {
            id: downloadIcon
            source: "qrc:/download-icon.svg"
            icon.width: 20
            icon.height: 20
            fill: CmnCfg.palette.black
            onClicked: fileChooser.open()

            FileDialog {
                id: fileChooser
                selectFolder: true
                folder: StandardPaths.writableLocation(
                            StandardPaths.DesktopLocation)
                onAccepted: Herald.utils.saveFile(path, fileUrl)
                selectExisting: false
            }
        }
    }
}
