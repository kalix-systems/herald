import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Pane {
    property var messageModel: Messages {
    }
    topPadding: 0 /// for better clipping behavior
    rightPadding: 0 /// for the scrollbar
    bottomPadding: 0
    ///--- chat view, shows messages
    ListView {

        boundsBehavior: Flickable.StopAtBounds
        anchors {
            right: parent.right
            bottom: chatBox.top
            top: parent.top
            left: parent.left
            topMargin: 20 /// allow one unit of spacing between ceiling and first message
            bottomMargin: 20 /// allow one unit of spacing between base and final message
        }
        spacing: 10
        model: messageModel
        ScrollBar.vertical: ScrollBar {
            id: chatScrollBar
            Component.onCompleted: {
                position = 1.0
            }
        }
        delegate: Column {
            readonly property bool outbound: author == config.id
            anchors {
                right: outbound ? parent.right : undefined
                rightMargin: chatScrollBar.width * 2
            }
            Row {
                TextMetrics {
                    id: messageMetrics
                    text: qsTr(body)
                }

                Rectangle {
                    id: recto
                    color: outbound ? "lightsteelblue" : "lightgrey"
                    radius: 10
                    width: labo.width + 10
                    height: labo.height + 10

                    Label {
                        property bool tooLong: (messageMetrics.width >= root.width / 4)
                        id: labo
                        wrapMode: Text.Wrap
                        width: tooLong ? root.width / 4 : messageMetrics.advanceWidth
                        text: messageMetrics.text
                        anchors.centerIn: recto
                    }
                }
            }
        }
    }

    ///--- Text entry area
    Rectangle {
        id: chatBox
        height: 50
        anchors {
            right: parent.right
            bottom: parent.bottom
            left: parent.left
        }
        TextArea {
            id: chatTextArea
            width: parent.width
            anchors {
                centerIn: parent
            }
            placeholderText: "Send a Message ..."
            background: Rectangle {
                radius: 100
                color: "gray"
            }
            Keys.onReturnPressed: {
                if (chatTextArea.text.length <= 0) {
                    return
                }
                messageModel.send_message(chatTextArea.text)
                chatScrollBar.position = 1.0
                chatTextArea.clear()
            }
        }
    }
}
