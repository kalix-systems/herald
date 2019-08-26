import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "ChatView" as CVUtils

Pane {
    id: chatPane
    enabled: false
    opacity: 0

    property var messageModel: Messages {
    }

    property alias messageBar: messageBar

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

    padding: 0

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

        ListView {
            anchors.fill: parent
            id: chatListView
            Component.onCompleted: forceActiveFocus()

            MouseArea {
                anchors.fill: parent
                onClicked: forceActiveFocus()
            }

            Keys.onUpPressed: chatScrollBar.decrease()
            Keys.onDownPressed: chatScrollBar.increase()

            boundsBehavior: Flickable.StopAtBounds
            spacing: QmlCfg.margin
            model: messageModel

            ///--- scrollbar for chat messages
            ScrollBar.vertical: ScrollBar {
                id: chatScrollBar
                size: 50
                Component.onCompleted: position = 1.0
            }

            delegate: Column {
                readonly property bool outbound: author === config.id

                anchors {
                    right: if (outbound) {
                               parent.right
                           }
                    rightMargin: chatScrollBar.width * 2
                }

                CVUtils.ChatBubble {
                    topPadding: if (index === 0) {
                                    QmlCfg.margin
                                } else {
                                    0
                                }
                    text: body
                }
            } /// Delegate
        } /// ListView
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
                    if (text.trim().length === 0)
                        return
                    messageModel.send_message(text)
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
