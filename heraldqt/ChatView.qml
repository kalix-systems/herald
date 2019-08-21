import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0

Pane {
    property var messageModel: Messages {
    }
    topPadding: 0 /// for better clipping behavior
    rightPadding: 0 /// for the scrollbar

    ///--- chat view, shows messages
    ListView {
        header: Rectangle {
            height: 10
        }
        boundsBehavior: Flickable.StopAtBounds
        anchors {
            right: parent.right
            bottom: chatBox.top
            top: parent.top
            left: parent.left
            bottomMargin: 20 ///allow one unit of spacing between base and final message
        }
        spacing: 20
        model: messageModel
        ScrollBar.vertical: ScrollBar {
            id: chatScrollBar
            Component.onCompleted: {
                position = 1.0
            }
        }
        delegate: Column {
            readonly property bool outbound: author == config.id
            height: messageText.height
            anchors {
                right: outbound ? parent.right : undefined
                rightMargin: 10
            }
            Row {
                anchors.right: parent.right
                Rectangle {
                    id: bubble
                    width: messageText.width + 10
                    height: messageText.height + 10
                    color: outbound ? "lightsteelblue" : "lightgrey"
                    radius: 10
                    Text {
                        anchors.centerIn: bubble
                        wrapMode: Text.WordWrap
                        id: messageText
                        text: qsTr(body)
                    }
                }
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
            if (chatBox.text.length <= 0) {
                return
            }
            messageModel.send_message(chatBox.text)
            chatScrollBar.position = 1.0
            chatBox.clear()
        }
    }
}
