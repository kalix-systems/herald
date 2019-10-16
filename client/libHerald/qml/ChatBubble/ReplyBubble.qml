import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

ColumnLayout {

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string opName: "@unknown"
    property string opBody: ""
    property color opColor: "gray"
    property string authorName: ""
    property int spacing: 0

    ChatLabel {
        id: sender
        senderName: authorName
    }

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: reply.implicitHeight
        color: Qt.lighter(QmlCfg.palette.tertiaryColor, 1.3)
        Layout.margins: QmlCfg.smallMargin
        Layout.topMargin: 0
        Layout.minimumWidth: reply.width

        Rectangle {}

        ColumnLayout {
            id: reply
            spacing: 1

            Label {
                id: opLabel
                text: opName
                font.bold: true
                Layout.margins: QmlCfg.smallMargin
                Layout.bottomMargin: 0
                Layout.preferredHeight: opName !== "" ? implicitHeight : 0
                color: QmlCfg.palette.mainTextColor
            }

            TextMetrics {
                readonly property real constWidth: replyBody.width * 3
                id: opBodyTextMetrics
                text: opBody
                elideWidth: constWidth
                elide: Text.ElideRight
            }

            StandardTextEdit {
                id: replyBody
                text: opBodyTextMetrics.elidedText
                Layout.minimumWidth: messageBody.width
            }
        }
    }

    StandardTextEdit {
        id: messageBody
    }

    StandardStamps {}
}
