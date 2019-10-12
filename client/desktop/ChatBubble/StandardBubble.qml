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
    property string authorName: ""

    spacing: 0

    Common.ChatLabel {
        id: sender
        senderName: authorName
    }

    StandardTextEdit {
    }

    StandardStamps {
    }
}
