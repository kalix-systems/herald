import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils
import "../ChatView" as ChatView

/// Displays an entity (user or group) avatar, name, and optional extra
/// information (username or message snippet, timestamp, read receipt).
Rectangle {
    id: entityItem

    // path to the user's profile picture, if one is set
    property string pfpPath
    // the name of the entity, displayed as the top label
    property string entityName: ''
    // the lower line(s) of text to display under the entity name
    property string subLabelText: ''
    // optional timestamp to display, if subLabelText is a message body
    property string timestamp
    // value of latest read receipt according to the ReceiptStatus enum
    property int lastReceipt
    // the index corresponding to the visual color of this entity
    property string color

    property real topTextMargin: CmnCfg.units.dp(8)
    property real bottomTextMargin: CmnCfg.units.dp(16)
    property bool isGroup: false

    color: CmnCfg.palette.white
    height: itemAvatar.size + 2 * CmnCfg.smallMargin

    anchors {
        left: parent.left
        right: parent.right
    }

    Avatar {
        id: itemAvatar
        anchors {
            left: parent.left
            verticalCenter: parent.verticalCenter
        }
        color: entityItem.color
        initials: Utils.initialize(entityName)
        size: CmnCfg.units.dp(48)
        isGroup: entityItem.isGroup
        pfpPath: entityItem.pfpPath
    }

    ConversationLabel {
        anchors {
            left: itemAvatar.right
            right: parent.right
            top: parent.top
            bottom: parent.bottom
            leftMargin: CmnCfg.defaultMargin
            topMargin: topTextMargin
            bottomMargin: bottomTextMargin

        }

        contactName: entityName
        lastBody: subLabelText
        lastTimestamp: timestamp
        lastReceipt: entityItem.lastReceipt
        labelFontSize: CmnCfg.labelSize
    }
}
