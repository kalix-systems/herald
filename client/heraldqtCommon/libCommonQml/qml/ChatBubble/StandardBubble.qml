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

    spacing: 0

    Label {
        id: sender
        Layout.margins: 5
        Layout.preferredHeight: text ? text.height : 0
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
            font.pixelSize: 10
            text: friendlyTimestamp
            id: timestamp
        }

        Item {
            Layout.fillWidth: true
        }

        Image {
            id: receipt
            source: receiptImage
            sourceSize: Qt.size(12, 12)
        }
    }
}
