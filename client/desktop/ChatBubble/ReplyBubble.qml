import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../common" as Common

ColumnLayout {

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string opName: "@unknown"
    property string opBody: ownedConversation.messageBodyById(op)
    property color opColor: "gray"
    property string authorName: ""
    property int spacing: 0

    Common.ChatLabel {
        id: sender
        senderName: authorName
    }

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: reply.implicitHeight
        color: outbound ? opColor : Qt.lighter(userColor, 1.2)
        radius: QmlCfg.radius / 2
        Layout.margins: QmlCfg.smallMargin
        Layout.topMargin: 0
        Layout.minimumWidth: reply.width
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
                color: outbound ? QmlCfg.palette.mainTextColor : QmlCfg.palette.iconFill
            }

            // KAAVYA: write our own ellision function
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
