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
    property string authorName: ""

    spacing: 0

    Label {
        id: sender
        text: authorName === "" ? "" : "@" + authorName
        Layout.margins: authorName === "" ? 0 : QmlCfg.smallMargin
        Layout.bottomMargin: authorName === "" ? QmlCfg.smallMargin : QmlCfg.margin
        Layout.preferredHeight: authorName !== "" ? QmlCfg.margin : 0
        font.bold: true
        color: outbound ? "black" : "white"
    }

    TextEdit {
        text: body
        Layout.maximumWidth: maxWidth
        Layout.leftMargin: QmlCfg.smallMargin
        Layout.rightMargin: QmlCfg.smallMargin
        wrapMode: Text.WrapAtWordBoundaryOrAnywhere
        Layout.alignment: Qt.AlignLeft
        selectByMouse: true
        selectByKeyboard: true
        readOnly: true
        color: outbound ? "black" : "white"
    }

    RowLayout {
        Layout.margins: QmlCfg.smallMargin
        Label {
            font.pixelSize: 10
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
            sourceSize: Qt.size(12, 12)
        }
    }
}
