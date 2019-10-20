import QtQuick 2.0
import LibHerald 1.0

Rectangle {
    id: contactItem

    // the group name or displayName of the conversation
    property string contactName
    // the previous message of the conversation, or the empty string
    property string lastMessage
    // the previous latest human readable timestamp, or the empty string
    property string lastTimestamp
    // the value of the latest read receipt according to the ReceiptStatus enum
    property int lastReceipt: 0

    height: QmlCfg.units.dp(50)
    color: QmlCfg.palette.mainColor
    border.color: QmlCfg.palette.secondaryColor

    // fill parent width
    anchors {
        left: parent.left
        right: parent.right
    }
}
