import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

ColumnLayout {
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string imageSource: ""
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

    Image {
        id: image
        property real aspectRatio: sourceSize.height / sourceSize.width
        Layout.maximumWidth: 400
        Layout.minimumWidth: 200
        Layout.preferredWidth: sourceSize.width
        Layout.maximumHeight: 300
        source: imageSource
        fillMode: Image.PreserveAspectCrop
        asynchronous: true
    }

    TextEdit {
        text: body
        Layout.leftMargin: QmlCfg.smallMargin
        Layout.rightMargin: QmlCfg.smallMargin
        font.pixelSize: QmlCfg.chatTextSize
        Layout.preferredHeight: body.length === 0 ? 0 : undefined
        Layout.maximumWidth: image.width - 10 // margin
        Layout.alignment: Qt.AlignLeft
        selectByMouse: true
        selectByKeyboard: true
        readOnly: true
        color: outbound ? "black" : "white"
        wrapMode: Text.WrapAtWordBoundaryOrAnywhere
    }

    RowLayout {
        Layout.margins: QmlCfg.smallMargin

        Label {
            text: friendlyTimestamp
            id: timestamp
            font.pixelSize: QmlCfg.chatTextSize
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
