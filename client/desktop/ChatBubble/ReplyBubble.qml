import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

// TODO: demagic with libherald import
// TODO: js for switching and choosing read receipts
ColumnLayout {

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string opName: "@unknown"
    property string opBody: ownedConversation.messageBodyById(op)
    property color opColor: "gray"
    property string authorName: ""
    property int

    spacing: 0

    Label {
        id: sender
        text: authorName === "" ? "" : "@" + authorName
        Layout.margins: authorName === "" ? 0 : QmlCfg.smallMargin
        Layout.bottomMargin: authorName === "" ? QmlCfg.smallMargin : QmlCfg.margin
        Layout.preferredHeight: authorName !== "" ? QmlCfg.margin : 0
        font.bold: true
    }

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: reply.implicitHeight
        color: outbound ? opColor : Qt.lighter(userColor, 1.2)
        radius: QmlCfg.radius / 2
        Layout.margins: 5
        Layout.topMargin: 0
        Layout.minimumWidth: reply.width
        ColumnLayout {
            id: reply
            spacing: 1
            Label {
                id: opLabel
                text: opName
                font.bold: true
                Layout.margins: 5
                Layout.bottomMargin: 0
                Layout.preferredHeight: opName !== "" ? implicitHeight : 0
            }

            TextMetrics {
                id: opBodyTextMetrics
                text: opBody
                elideWidth: (maxWidth - QmlCfg.smallMargin) * 2
                elide: Text.ElideRight
            }

            TextEdit {
                Layout.margins: 5
                text: opBodyTextMetrics.elidedText
                Layout.maximumWidth: Math.max(maxWidth, 200)
                Layout.minimumWidth: messageBody.width
                selectByMouse: true
                selectByKeyboard: true
                readOnly: true
                wrapMode: Text.WrapAtWordBoundaryOrAnywhere
            }
        }
    }



    TextEdit {
        id: messageBody
        text: body
        Layout.leftMargin: 5
        Layout.rightMargin: 5
        Layout.maximumWidth: Math.max(parent.maxWidth, 200)
        Layout.alignment: Qt.AlignLeft
        selectByMouse: true
        selectByKeyboard: true
        readOnly: true
        color: outbound ? "black" : "white"
        wrapMode: Text.WrapAtWordBoundaryOrAnywhere
    }

    RowLayout {
        Layout.margins: 5

        Label {
            font.pixelSize: QmlCfg.chatTextSize
            text: friendlyTimestamp
            id: timestamp
            color: outbound ? "black" : "white"
        }

        Item {
            Layout.fillWidth: true
        }

        Image {
            id: receipt
            source: receiptImage
            sourceSize: Qt.size(16, 16)
        }
    }
}
