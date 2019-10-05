import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13

Rectangle {
    id: background
    property color bubbleColor: "light green"
    property string body: ""
    property real cfgMargins: 10
    property real cfgSmallMargins: cfgMargins / 2
    property real cfgRadius: 10
    property int epochtimestamp_ms: 100
    property int maxWidth: parent.width * 0.66
    property Component additionalContent
    color: bubbleColor
    width: bubbleLayout.width
    height: bubbleLayout.height
    radius: cfgRadius

    ColumnLayout {
        id: bubbleLayout
        spacing: 0

        Loader {
            id: adContent
            active: additionalContent
            Layout.topMargin: active ? 10 : 0
            Layout.bottomMargin: active ? 10 : 0
            sourceComponent: additionalContent
        }

        TextEdit {
            id: text
            Layout.preferredWidth: maxWidth
            Layout.maximumWidth: adContent.active ? adContent.width - 10 : undefined
            Layout.minimumWidth: 200
            Layout.margins: body.length > 0 ? cfgSmallMargins : 0
            text: body
            readOnly: true
            wrapMode: TextEdit.Wrap
        }

        Label {
            id: timestamp
            text: epochtimestamp_ms
            Layout.maximumWidth: maxWidth
            Layout.margins: cfgSmallMargins
            font.pixelSize: 10
            color: "grey"
        }
    }
}
