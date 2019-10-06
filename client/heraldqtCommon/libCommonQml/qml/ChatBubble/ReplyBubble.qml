import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

// TODO: demagic with libherald import
// TODO: js for switching and choosing read receipts
ColumnLayout {

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string opName: "@unknown"
    property string opBody: ""
    property color opColor: "gray"

    spacing: 0

    Label {
        id: sender
        Layout.margins: 5
        Layout.preferredHeight: text ? text.height : 0
    }

    Rectangle {
        id: replyWrapper
        height: reply.height
        color: opColor
        radius: 10
        Layout.margins: 5
        Layout.topMargin: 0
        Layout.minimumWidth: reply.width
        ColumnLayout {
            id: reply
            spacing: 1
            Label {
                id: opLabel
                text: opName
                Layout.margins: 5
                Layout.bottomMargin: 0
                Layout.preferredHeight: text ? text.height : 0
            }

            TextEdit {
                text: "something logner than the original message"
                Layout.margins: 5
                Layout.maximumWidth: Math.max(parent.maxWidth, 200)
                wrapMode: Text.WrapAtWordBoundaryOrAnywhere
            }
        }
    }

    TextEdit {
        text: body
        Layout.leftMargin: 5
        Layout.rightMargin: 5
        Layout.maximumWidth: Math.max(parent.maxWidth, 200)
        wrapMode: Text.WrapAtWordBoundaryOrAnywhere
    }

    RowLayout {
        Layout.margins: 5

        Label {
            text: friendlyTimestamp
            id: timestamp
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
