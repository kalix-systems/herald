import QtQuick 2.13
import "../../js/ContactView.mjs" as JS
import "../../../../foundation/js/utils.mjs" as Utils
import LibHerald 1.0

// KAAVYA 1: this is bad, and paul watches jimmy neutron.
Item {
    property string label: "unknown"
    property string summaryText: ""

    Text {
        id: displayName
        text: label
        font.bold: true
        clip: true
        color: CmnCfg.palette.secondaryColor
        elide: Text.ElideRight
        anchors {
            top: parent.top
            left: parent.left
            right: timestampLabel.left
            margins: CmnCfg.margin
        }
    }

    Text {
        id: summary
        text: summaryText.trim().split("\n")[0]
        color: CmnCfg.palette.secondaryColor
        elide: Text.ElideRight
        anchors {
            top: displayName.bottom
            left: parent.left
            right: receiptIcon.left
            margins: CmnCfg.margin
            topMargin: CmnCfg.smallMargin
        }
    }

    Text {
        id: timestampLabel
        text: Utils.friendlyTimestamp(messageModel.lastEpochTimestampMs)
        font.pixelSize: 10
        color: CmnCfg.palette.secondaryTextColor
        anchors {
            top: parent.top
            right: parent.right
            margins: CmnCfg.margin
        }
    }

    Image {
        id: receiptIcon
        sourceSize: Qt.size(24, 24)
        height: 16
        width: 16
        //ToDo: this value is always undefined
        source: JS.receiptStatusSwitch(messageModel.lastStatus)
        anchors {
            right: parent.right
            bottom: parent.bottom
            margins: CmnCfg.margin
            bottomMargin: CmnCfg.smallMargin
        }
    }
}
