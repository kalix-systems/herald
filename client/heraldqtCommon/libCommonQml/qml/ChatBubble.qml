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
    property Item additionalContent: Item {}

    height: innerLayout.height
    Layout.maximumWidth: maxWidth
    Rectangle {
        id: background
        radius: cfgRadius
        color: bubbleColor
        implicitWidth: innerLayout.width
        Layout.fillHeight: true
        Layout.fillWidth: true
        ColumnLayout {
            id: innerLayout

            Layout.fillHeight: true
            Layout.fillWidth: true

            Loader {
                id: additionalContentLoader
                sourceComponent: additionalContent
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
    }
}
