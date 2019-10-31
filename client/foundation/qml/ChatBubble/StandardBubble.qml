import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

ColumnLayout {

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property string body: ""
    property string friendlyTimestamp: ""
    property string receiptImage: ""
    property string authorName: ""
    property color authorColor
    spacing: 0

    Row {
        Layout.margins: CmnCfg.smallMargin / 2
        Layout.bottomMargin: 0
        spacing: CmnCfg.smallMargin / 2

        ChatLabel {
            id: uname
            senderName: authorName
            senderColor: authorColor
        }

        Label {
            id: timestamp
            text: friendlyTimestamp
            color: CmnCfg.palette.secondaryTextColor
            font.pixelSize: 10
            anchors {
                top: parent.top
                topMargin: 3
            }
        }
    }

    StandardTextEdit {}
    StandardStamps {}
}
