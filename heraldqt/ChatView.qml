import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "ChatView" as CVUtils

Pane {
    id: chatPane
    enabled: false
    opacity: 0
    padding: 0
    property alias messageBar: messageBar
    property Messages messageModel: Messages {
    }

    CVUtils.ChatBar {
        id: messageBar
    }

    ///--- border between messageBar and main chat view
    Rectangle {
        height: 1
        color: QmlCfg.palette.secondaryColor
        anchors {
            top: messageBar.bottom
            left: parent.left
            right: parent.right
        }
    }

    ///--- chat view, shows messages
    ScrollView {
        bottomPadding: QmlCfg.margin * 2
        clip: true
        anchors {
            top: messageBar.bottom
            bottom: chatTextAreaScroll.top
            left: parent.left
            right: parent.right
        }

        ///--- scrollbar for chat messages
        ScrollBar.vertical: ScrollBar {
            id: chatScrollBar
            Component.onCompleted: position = 1.0
            height: parent.height
            anchors.right: parent.right
        }

        Column {
            width: chatPane.width
            spacing: QmlCfg.margin

            Repeater {
                anchors.fill: parent
                id: chatListView
                Component.onCompleted: forceActiveFocus()
                model: messageModel

                MouseArea {
                    anchors.fill: parent
                    onClicked: forceActiveFocus()
                }

                Keys.onUpPressed: chatScrollBar.decrease()
                Keys.onDownPressed: chatScrollBar.increase()

                delegate: Column {
                    readonly property bool outbound: author === config.config_id

                    anchors {
                        right: if (outbound) {
                                   return parent.right
                               }
                        rightMargin: chatScrollBar.width * 1.5
                    }

                    CVUtils.ChatBubble {
                        topPadding: if (index === 0) {
                                        return QmlCfg.margin
                                    } else {
                                        return 0
                                    }
                        text: body
                    }
                } /// Delegate
            } /// ListView
        }
    }

    ///--- Text entry area
    ScrollView {
        clip: true
        id: chatTextAreaScroll
        anchors {
            right: parent.right
            bottom: parent.bottom
            left: parent.left
        }
        background: Rectangle {
            color: QmlCfg.palette.mainColor
        }
        height: Math.min(contentHeight, 100)

        TextArea {
            id: chatText
            background: Rectangle {
                color: QmlCfg.palette.secondaryColor
                anchors {
                    fill: parent
                    margins: QmlCfg.margin / 2
                }
                radius: QmlCfg.radius
            }
            padding: QmlCfg.margin
            wrapMode: TextEdit.WrapAtWordBoundaryOrAnywhere
            placeholderText: "Send a Message ..."
            Keys.onReturnPressed: {
                if (event.modifiers & Qt.ShiftModifier) {
                    chatText.text = chatText.text + "\n"
                    chatText.cursorPosition = chatText.text.length
                } else {
                    if (text.length <= 0) {
                        return
                    }
                    if (text.trim().length === 0) {
                        return
                    }
                    var result = networkHandle.send_message(text,
                                               messageModel.conversationId)
                    messageModel.insert_message(text, result)
                    chatScrollBar.position = 1.0
                    clear()
                }
            }
            Keys.onEscapePressed: {
                chatListView.forceActiveFocus()
            }
        } /// Chat entry field
    } /// scroll area

    states: State {
        name: "visibleview"
        PropertyChanges {
            target: chatPane
            opacity: 100
            enabled: true
        }
    }
}
