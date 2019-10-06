import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

ColumnLayout {
    property real maxWidth: Math.min(parent.maxWidth, 600)
    property string body: ""
    property int epochtimestamp_ms: 100
    property int receiptCode: 0
    property string imageSource: ""

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
            text: "100"
            id: timestamp
        }

        Item {
            Layout.fillWidth: true
        }

        Image {
            id: receipt
            source: "file:../../icons/double-check-receipt-icon.svg"
            sourceSize: Qt.size(16, 16)
        }
    }
}
