import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0

Rectangle {
    id: background
    property color bubbleColor: CmnCfg.palette.paneColor
    property color senderColor: "white"
    property string receiptImage: ""
    property string friendlyTimestamp: ""
    property bool outbound: parent.outbound
    property real maxWidth: 0.0
    property bool highlight: false
    property Component content

    color: bubbleColor
    width: contentLoader.width
    height: contentLoader.height


    //two rectangles to extend to both sides of pane
    Rectangle {
        width: convWindow.width
        anchors.right: parent.right
        color: CmnCfg.palette.sideBarHighlightColor
        anchors.verticalCenter: parent.verticalCenter
        height: parent.height + CmnCfg.smallMargin
        visible: highlight
        z: -1
    }

    Rectangle {
        width: convWindow.width
        anchors.left: parent.right
        color: CmnCfg.palette.sideBarHighlightColor
        anchors.verticalCenter: parent.verticalCenter
        height: parent.height + CmnCfg.smallMargin
        visible: highlight
        z: -1
    }

    Rectangle {
        id: verticalAccent
        anchors.right: !outbound ? contentLoader.left : undefined
        anchors.left: outbound ? contentLoader.right : undefined
        height: contentLoader.height
        width: CmnCfg.smallMargin / 4
        color: senderColor
    }

    Loader {
        id: contentLoader
        property int maxWidth: parent.maxWidth
        sourceComponent: content
    }

    Item {
        anchors.bottom: parent.bottom
    }
}
