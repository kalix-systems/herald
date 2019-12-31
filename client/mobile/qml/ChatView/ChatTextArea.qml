import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import "qrc:/imports/ChatBubble" as CB
import LibHerald 1.0
import "../Common"

ColumnLayout {
    id: chatRowLayout
    readonly property var select: function () {
        cta.forceActiveFocus()
    }

    property bool send: cta.text.length > 0
    property string chatName: 'conversation'
    width: parent.width
    spacing: 0
    Loader {
        id: replyLoader
        Layout.fillWidth: true
        Layout.margins: CmnCfg.smallMargin
        Layout.preferredHeight: item ? item.height : 0
        active: ownedMessages.builder.isReply
        sourceComponent: CB.ComposeReplyComponent {
            builderData: ownedMessages.builder
        }
    }

    RowLayout {
        Layout.fillWidth: true
        TextArea {
            id: cta
            Layout.fillWidth: true
            placeholderText: qsTr('Message ') + chatRowLayout.chatName
            wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere
            color: CmnCfg.palette.black
            selectionColor: CmnCfg.palette.highlightColor
            font {
                pixelSize: CmnCfg.chatTextSize
                family: CmnCfg.chatFont.name
            }
            Keys.onPressed: if ((event.key === Qt.Key_Backspace
                                 || event.key === Qt.Key_Delete)
                                    && cta.text.length === 0) {
                                Qt.inputMethod.hide()
                            }
        }
        Grid {
            // TODO: Collapse options into plus when typing
            // TODO: this is a binding loop use TextMetrics
            columns: cta.lineCount > 1 ? 1 : 2
            spacing: CmnCfg.defaultMargin
            AnimIconButton {
                color: CmnCfg.palette.black
                imageSource: "qrc:/camera-icon.svg"
            }
            AnimIconButton {
                color: CmnCfg.palette.black
                onClicked: if (send) {
                               ownedMessages.builder.body = cta.text
                               ownedMessages.builder.finalize()
                               cta.clear()
                           }
                imageSource: send ? "qrc:/send-icon.svg" : "qrc:/plus-icon.svg"
            }
        }
    }
}
