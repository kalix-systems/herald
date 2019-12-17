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
    height: childrenRect.height
    width: childrenRect.width
    spacing: CmnCfg.smallMargin / 2

    ListView {
        id: fileList
        anchors.top: parent.top
        interactive: false
        width: imageAttach ? 300 - downloadList.width : contentItem.childrenRect.width
        height: contentItem.childrenRect.height
        spacing: CmnCfg.smallMargin / 2

        delegate: RowLayout {
            clip: true
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
                Layout.maximumWidth: imageAttach ? 300 - CmnCfg.smallMargin * 2
                                                   - fileSize.width - fileIcon.width
                                                   - downloadList.width : bubbleRoot.maxWidth
                                                   - fileSize.width - fileIcon.width
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
