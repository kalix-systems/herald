import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Window 2.13
import "./ChatBubble.qml"

Window {

    width: 650
    height: 500

    ChatBubble {
        anchors.centerIn: parent
        body: "orldWorldHello  WorldHelloWorldHello World"
        epochtimestamp_ms: 100
        bubbleColor: "light gray"
    }
}
