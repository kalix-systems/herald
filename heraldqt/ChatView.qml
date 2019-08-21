import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0

Pane {
    property var messageModel: Messages {
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
            id: chatScrollBar
            x: parent.width //TODO : not hardcode this.
            Component.onCompleted: {
                position = 1.0
            }
        }
        delegate: Item {
            height: messageText.height
            Row {
                Rectangle {
                    id: bubble
                    width: messageText.width + 10
                    height: messageText.height + 10
                    color: "lightgrey"
                    radius: 10
                    Text {
                        anchors.centerIn: bubble
                        wrapMode: Text.WordWrap
                        id: messageText
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
            chatScrollBar.position = 1.0
            chatBox.clear()
        }
    }
}
