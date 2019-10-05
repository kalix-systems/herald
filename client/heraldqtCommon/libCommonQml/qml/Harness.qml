import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Window 2.13

Window {
    id: window

    width: 650
    height: 500

    ChatBubble {
        id: thing
        anchors.right: parent.right
        anchors.margins: 10
        body: " abu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuu bu dabuuu"
        epochtimestamp_ms: 100
        bubbleColor: "light gray"
        additionalContent: ImageContent {
            source: "https://www.audubon.org/sites/default/files/a1_4202_1_fish-crow_alejandra_lewandowski_kk.jpg"
        }
    }
    ChatBubble {
        anchors.top: thing.bottom
        anchors.right: parent.right
        anchors.margins: 10
        body: " abu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuuabu dabuuu bu dabuuu"
        epochtimestamp_ms: 100
        bubbleColor: "light gray"
        reply: {
            text: "greese"
            op: "bob"
        }
    }
}
