import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13

ColumnLayout {
    id: bubbleLayout

    property color bubbleColor: "light green"
    property string body: "test body"
    property real cfgMargins: 10
    property real cfgSmallMargins: cfgMargins / 2
    property real cfgRadius: 10
    property int epochtimestamp_ms: 100
    property int maxWidth: parent.width * 0.66
    property Component additionalContent
    Layout.maximumWidth: maxWidth

    ColumnLayout {
        z: 1
        id: innerLayout

        Loader {
            property int contentHeight: 200
            property int contentWidth: 300
            //            property var contentSource: ["qrc:/mary.png", "qrc:/land.png"]
            id: temp
            sourceComponent: additionalContent
            Layout.topMargin: additionalContent ? cfgMargins : 0
            Layout.alignment: Qt.AlignCenter
        }
        TextEdit {
            Layout.maximumWidth: maxWidth
            Layout.margins: cfgSmallMargins
            text: body
            readOnly: true
            wrapMode: TextEdit.Wrap
        }

        Label {
            id: timestamp
            text: epochtimestamp_ms
            Layout.margins: cfgSmallMargins
            font.pixelSize: 10
            color: "grey"
        }
    }

    Rectangle {
        z: 0
        color: bubbleColor
        anchors.fill: innerLayout
        radius: cfgRadius
    }
}
