import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

RowLayout {
    Layout.margins: QmlCfg.smallMargin
    Label {
        font.pixelSize: 10
        text: friendlyTimestamp
        id: timestamp
        color: outbound ? QmlCfg.palette.mainTextColor : QmlCfg.palette.iconFill

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
