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
    property var reply: undefined
    property Component additionalContent

    color: bubbleColor
    width: bubbleLayout.width
    height: bubbleLayout.height
    radius: cfgRadius

    ColumnLayout {
        id: bubbleLayout
        spacing: 0

        Loader {
            id: imageContent
            active: additionalContent
            Layout.topMargin: active ? cfgMargins : 0
            Layout.bottomMargin: active ? cfgMargins : 0
            sourceComponent: additionalContent
        }

        Loader {
            id: replyContent
            active: reply !== undefined
            Layout.margins: active ? cfgSmallMargins : 0
            sourceComponent: ReplyContent {
                text: reply.text
                op: reply.op
                color: reply.color
                messageId: reply.messageId
            }
        }

        TextEdit {
            id: text
            Layout.preferredWidth: maxWidth
            Layout.maximumWidth: imageContent.active ? imageContent.width - cfgMargins : undefined
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
