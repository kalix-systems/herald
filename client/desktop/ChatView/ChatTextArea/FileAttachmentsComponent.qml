import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.12
import "qrc:/common" as Common
import LibHerald 1.0
import "qrc:/imports" as Imports
import "qrc:/imports/js/utils.mjs" as Utils

ScrollView {
    width: parent.width
    height: wrapperRow.height
    ScrollBar.horizontal: ScrollBar {
        policy: ScrollBar.AsNeeded
    }

    Row {
        id: wrapperRow
        height: 28
        Layout.margins: CmnCfg.mediumMargin
        width: parent.width
        spacing: 5
        Repeater {
            id: fileRepeater
            model: ownedConversation.builder.documentAttachments
            delegate: RowLayout {
                clip: true
                Imports.ButtonForm {
                    id: fileIcon
                    icon.source: "qrc:/file-icon.svg"
                    icon.height: 20
                    icon.width: height
                }
                Text {
                    id: fileName
                    color: CmnCfg.palette.black
                    text: documentAttachmentName
                    font.family: CmnCfg.chatFont.name
                    font.pixelSize: 13
                    font.weight: Font.Medium
                    elide: Text.ElideMiddle
                    Layout.maximumWidth: chatTextArea.width - fileSize.width
                                         - fileIcon.width - CmnCfg.smallMargin * 2
                }

                Text {
                    id: fileSize
                    text: Utils.friendlyFileSize(documentAttachmentSize)
                    font.family: CmnCfg.chatFont.name
                    font.pixelSize: 10
                    font.weight: Font.Light
                    color: CmnCfg.palette.darkGrey
                }
            }
        }
    }
}
