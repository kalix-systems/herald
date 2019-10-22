import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0

Rectangle {
    id: background
    property color bubbleColor: QmlCfg.palette.secondaryColor
    property color senderColor: "white"
    property string receiptImage: ""
    property string friendlyTimestamp: ""
    property bool outbound: parent.outbound
    property real maxWidth: 0.0
    property Component content

    color: bubbleColor
    width: contentLoader.width
    height: contentLoader.height

    Rectangle {
        id: verticalAccent
        anchors.right: !outbound ? contentLoader.left : undefined
        anchors.left: outbound ? contentLoader.right : undefined
        height: parent.height
        width: QmlCfg.margin
        color: senderColor
    }

    Loader {
        id: contentLoader
        property int maxWidth: parent.maxWidth
        sourceComponent: content
    }
}
