import QtQuick 2.13
import "./ContactView.mjs" as JS
import "../common/utils.mjs" as Utils
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
        color: QmlCfg.palette.mainTextColor
        elide: Text.ElideRight
        anchors {
            top: parent.top
            left: parent.left
            right: timestampLabel.left
            margins: QmlCfg.margin
        }
    }

    Text {
        id: summary
        text: summaryText.trim().split("\n")[0]
        color: QmlCfg.palette.secondaryTextColor
        elide: Text.ElideRight
        anchors {
            top: displayName.bottom
            left: parent.left
            right: receiptIcon.left
            margins: QmlCfg.margin
            topMargin: QmlCfg.smallMargin
        }
    }

    Text {
        id: timestampLabel
        text: Utils.friendlyTimestamp(messageModel.lastEpochTimestampMs)
        font.pixelSize: 10
        color: QmlCfg.palette.secondaryTextColor
        anchors {
            top: parent.top
            right: parent.right
            margins: QmlCfg.margin
        }
    }

    Image {
        id: receiptIcon
        sourceSize: Qt.size(24, 24)
        height: 16
        width: 16
        source: JS.receiptStatusSwitch(
                    messageModel.lastStatus) //ToDo: this value is alwasy undefined
        anchors {
            right: parent.right
            bottom: parent.bottom
            margins: QmlCfg.margin
            bottomMargin: QmlCfg.smallMargin
        }
    }
}
