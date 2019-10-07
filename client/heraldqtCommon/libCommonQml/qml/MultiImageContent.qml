import QtQuick 2.13
import QtQuick.Layouts 1.12

Item {
    id: multipleImageView
    property var sourceItems: ["qrc:/mary.png", "qrc:/mary.png"]
    property var cfgMargin: 10
    property real firstItemHeight: 300
    property real firstItemWidth: 500
    Rectangle {
        id: upper
        z: 3
        height: parent.height - cfgMargin
        width: parent.width - cfgMargin - label.width
        x: 10
        y: 10
        anchors {
            margins: cfgMargin
        }
        color: "gray"
    }
    Rectangle {
        z: 2
        height: parent.height - cfgMargin * 2.0
        width: parent.width - cfgMargin * 2.0 - label.width
        anchors {
            margins: cfgMargin
        }

        color: "black"
    }
    Image {}
    Column {
        id: label
        anchors.left: upper.right
        anchors.margins: parent.width - upper.width
        anchors.verticalCenter: multipleImageView.verticalCenter
        Text {
            id: count
            text: "+ " + sourceItems.length
            font.bold: true
        }
        Text {
            anchors.horizontalCenter: count.horizontalCenter
            text: "More"
            font.bold: true
        }
    }
}
