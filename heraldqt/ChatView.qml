import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "ChatView"

Pane {
    id: chatPane
    property var messageModel: Messages {
    }

    property alias messageBar: messageBar

    ChatBar {
        id: messageBar
    }

    padding: 0

    ///--- chat view, shows messages
    ScrollView {
        bottomPadding: 20
        clip: true
        anchors {
            top: messageBar.bottom
            bottom: chatTextAreaScroll.top
            left: parent.left
            right: parent.right
        }


        ListView {
            anchors {
                fill: parent
            }
            Component.onCompleted: forceActiveFocus()

            Keys.onUpPressed: {
                console.log("AAH")
                chatScrollBar.decrease() }

            Keys.onDownPressed: chatScrollBar.increase()

            boundsBehavior: Flickable.StopAtBounds
            spacing: 10
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
                    right: outbound ? parent.right : undefined
                    rightMargin: chatScrollBar.width * 2
                }

                ChatBubble {
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
            background: Rectangle {
                color: QmlCfg.palette.secondaryColor
                anchors {
                    fill: parent
                    margins: 5
                }
                radius: QmlCfg.radius
            }
            padding: 10
            wrapMode: TextEdit.WrapAtWordBoundaryOrAnywhere
            placeholderText: "Send a Message ..."
            Keys.onReturnPressed: {
                if (text.length <= 0)
                    return
                messageModel.send_message(text)
                chatScrollBar.position = 1.0
                clear()
            }
        } /// Chat entry field
    } /// scroll area
}
