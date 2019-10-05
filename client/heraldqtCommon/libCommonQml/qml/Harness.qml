import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Window 2.13

Window {
    id: window

    width: 650
    height: 500

    ChatBubble {
        anchors.verticalCenter: parent.verticalCenter
        anchors.right: parent.right
        anchors.margins: 10
        body: " abu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuu bu dabuuu"
        epochtimestamp_ms: 100
        bubbleColor: "light gray"
        additionalContent: ImageContent {
            source: "http://clipart-library.com/data_images/87233.png"
        }
    }
}
