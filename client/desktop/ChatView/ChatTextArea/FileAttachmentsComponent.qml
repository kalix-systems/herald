import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.12
import "qrc:/common" as Common
import LibHerald 1.0
import "qrc:/imports" as Imports
import "qrc:/imports/js/utils.mjs" as Utils

ScrollView {
    width: chatTextArea.width
    height: 30
    focus: true
    clip: true
    ScrollBar.horizontal.policy: contentWidth > width ? ScrollBar.AlwaysOn : ScrollBar.AlwaysOff

    Row {
        id: wrapperRow
        height: 24
        Layout.margins: CmnCfg.largeMargin
        width: parent.width
        spacing: CmnCfg.largeMargin
        Repeater {
            id: fileRepeater
            model: ownedConversation.builder.documentAttachments
            delegate: RowLayout {
                clip: true
                spacing: CmnCfg.microMargin
                Imports.ButtonForm {
                    id: clearFile
                    source: "qrc:/x-icon.svg"
                    Layout.alignment: Qt.AlignVCenter
                    Layout.topMargin: 1
                    onClicked: ownedConversation.builder.removeDoc(index)
                    fill: CmnCfg.palette.black
                    opacity: 1.0
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
                    Layout.alignment: Qt.AlignVCenter
                    elide: Text.ElideMiddle
                    Layout.maximumWidth: chatTextArea.width - fileSize.width
                                         - clearFile.width - CmnCfg.smallMargin * 2
                }

                Text {
                    id: fileSize
                    text: Utils.friendlyFileSize(documentAttachmentSize)
                    font.family: CmnCfg.chatFont.name
                    font.pixelSize: 10
                    font.weight: Font.Light
                    color: CmnCfg.palette.darkGrey
                    Layout.alignment: Qt.AlignVCenter
                }
            }
        }
    }
}
