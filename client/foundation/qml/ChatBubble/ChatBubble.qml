import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0

Rectangle {
    id: background

    readonly property color bubbleColor: CmnCfg.palette.lightGrey
    readonly property color senderColor: bubbleContent.authorColor
    readonly property bool highlight: messageModelData.matchStatus === 2

    property bool outbound: parent.outbound

    //property real maxWidth: 0.0
    property alias highlightItem: bubbleHighlight
    property Item convContainer

    property alias defaultWidth: bubbleContent.defaultWidth
    property var messageModelData

    color: bubbleColor
    width: bubbleContent.width
    height: bubbleContent.height

    //two rectangles to extend to both sides of pane
    Item {
        id: bubbleHighlight
        anchors.fill: parent
        z: -1
        opacity: highlight == true ? 1.0 : 0.0
        Rectangle {
            width: convContainer.width
            anchors.right: parent.right
            color: CmnCfg.palette.medGrey
            anchors.verticalCenter: parent.verticalCenter
            height: parent.height + CmnCfg.smallMargin
        }

        Rectangle {
            width: convContainer.width
            anchors.left: parent.right
            color: CmnCfg.palette.medGrey
            anchors.verticalCenter: parent.verticalCenter
            height: parent.height + CmnCfg.smallMargin
        }
    }

    Rectangle {
        id: verticalAccent
        anchors.right: !outbound ? bubbleContent.left : undefined
        anchors.left: outbound ? bubbleContent.right : undefined
        height: bubbleContent.height
        width: CmnCfg.smallMargin / 4
        color: senderColor
    }

    BubbleContent {
        id: bubbleContent
    }

    Item {
        anchors.bottom: parent.bottom
    }
}
