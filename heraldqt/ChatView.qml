import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0

Pane {
    property string chatId: "empty"
    ///--- Rust model for messages and use ID's
    Messages {
        id: messageModel
        conversationId: chatId
    }
    ///--- chat view, shows messages
    ListView {
        boundsBehavior: Flickable.StopAtBounds
        anchors {
            right: parent.right
            bottom: chatBox.top
            top: parent.top
            left: parent.left
        }
        spacing: 20
        model: messageModel
        ScrollBar.vertical: ScrollBar {
        }
        delegate: Item {
            height: text.height
            Row {
                Rectangle {
                    id: bubble
                    width: text.width + 10
                    height: text.height + 10
                    color: "lightgrey"
                    radius: 10
                    Text {
                        anchors.centerIn: bubble
                        wrapMode: Text.WordWrap
                        id: text
                        text: qsTr(body)
                    }
                }
                anchors.verticalCenter: parent.verticalCenter
            }
        }
    }

    ///--- Text entry area
    TextArea {
        id: chatBox
        anchors {
            right: parent.right
            bottom: parent.bottom
            left: parent.left
        }
        placeholderText: "Send a Message ..."
        background: Rectangle {
            radius: 100
            color: "gray"
        }
        Keys.onReturnPressed: {
            messageModel.send_message(chatBox.text)
            chatBox.clear()
        }
    }
}
