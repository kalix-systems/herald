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


    ChatLabel {
        id: uname
        senderName: authorName
        senderColor: authorColor
    }


    StandardTextEdit {}
    StandardStamps {}
}
