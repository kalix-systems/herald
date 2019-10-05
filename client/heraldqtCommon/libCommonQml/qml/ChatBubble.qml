import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13

Rectangle {
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
        Layout.minimumWidth: content.width

        Loader {
            id: content
            Layout.topMargin: active ? 10 : 0
            Layout.bottomMargin: active ? 10 : 0
            Layout.alignment: Qt.AlignCenter
            Layout.preferredWidth: additionalContent.width
            Layout.maximumWidth: maxWidth
            Layout.minimumWidth: 10
            Layout.maximumHeight: 400
            sourceComponent: additionalContent
        }

        TextEdit {
            Layout.preferredWidth: maxWidth
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
