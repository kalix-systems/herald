import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Window 2.13

Window {
    id: window

    width: 650
    height: 500

    ChatBubble {
        anchors.verticalCenter: parent.verticalCenter
        anchors.horizontalCenter: parent.horizontalCenter
        body: "world WorldHello  WorldHello WorldHello World"
        epochtimestamp_ms: 100
        bubbleColor: "light gray"
        additionalContent: MultiImageContent {}
    }
}
