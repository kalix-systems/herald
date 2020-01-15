import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import "qrc:/imports/ChatBubble" as CB
import LibHerald 1.0
import "../Common"

Column {
    id: chatRowLayout
    readonly property var select: function () {
        cta.forceActiveFocus()
    }

    property bool send: (cta.preeditText.length !== 0 || cta.text.length !== 0)
    property string chatName: 'conversation'
    width: parent.width
    spacing: 0
    Loader {
        id: replyLoader
        width: parent.width
        anchors.margins: CmnCfg.smallMargin
        height: item ? item.height : 0
        active: ownedMessages.builder.isReply
        sourceComponent: CB.ComposeReplyComponent {
            builderData: ownedMessages.builder
        }
    }

    Item {
        width: parent.width
        height: cta.height
        TextArea {
            id: cta
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
            anchors.left: parent.left
            anchors.right: buttons.left
        }
        Grid {
            // TODO: Collapse options into plus when typing
            // TODO: this is a binding loop use TextMetrics
            columns: 2
            anchors.right: parent.right
            anchors.bottom: parent.bottom
            anchors.bottomMargin: CmnCfg.microMargin
            id: buttons
            spacing: CmnCfg.defaultMargin
            AnimIconButton {
                color: CmnCfg.palette.black
                imageSource: "qrc:/camera-icon.svg"
                onTapped: if (Qt.platform.os === "ios")
                              mobHelper.launch_camera_dialog()
            }
            AnimIconButton {
                color: CmnCfg.palette.black
                onTapped: if (send) {
                              Qt.inputMethod.commit()
                              ownedMessages.builder.body = cta.text
                              cta.focus = true
                              ownedMessages.builder.finalize()
                              cta.clear()
                          } else {
                              if (Qt.platform.os === "ios")
                                  mobHelper.launch_file_picker()
                          }

                imageSource: send ? "qrc:/send-icon.svg" : "qrc:/plus-icon.svg"
            }
        }
    }
}
