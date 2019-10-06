import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Window 2.13
import "./ChatBubble" as CB

Window {
    id: window

    width: 650
    height: 500
    Column {
        spacing: 10
        anchors.right: parent.right
        anchors.margins: 10
        CB.ChatBubble {
            id: thing

            epochtimestamp_ms: 100
            receiptCode: 0
            bubbleColor: "light gray"
            content: CB.StandardBubble {
                body: "test text that is"
            }
        }

        CB.ChatBubble {
            anchors.margins: 10
            epochtimestamp_ms: 100
            receiptCode: 0
            bubbleColor: "light gray"
            content: CB.ReplyBubble {
                body: "test text that is"
            }
        }

        CB.ChatBubble {
            anchors.margins: 10
            epochtimestamp_ms: 100
            receiptCode: 0
            bubbleColor: "light gray"
            content: CB.ImageBubble {
                imageSource: "https://via.placeholder.com/200x200/100"
                body: "test text that is the fuck are you talking about"
            }
        }
    }
}
