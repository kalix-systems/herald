import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

RowLayout {
    Layout.leftMargin: CmnCfg.smallMargin
    Layout.rightMargin: CmnCfg.smallMargin
    Layout.topMargin: 0
    Layout.bottomMargin: CmnCfg.smallPadding

    Row {
        spacing: 2
        Label {
            id: timestamp
            font.pixelSize: 10
            text: friendlyTimestamp
            color: CmnCfg.palette.secondaryTextColor
        }

        Button {
            id: clock
            icon.source: conversationItem.expirationPeriod
                         !== 0 ? "qrc:/countdown-icon-temp.svg" : ""
            icon.height: 16
            icon.width: 16
            icon.color: "grey"
            padding: 0
            background: Item {
            }
            anchors.verticalCenter: timestamp.verticalCenter
        }
    }

    Item {
        Layout.fillWidth: true
    }

    Button {
        id: receipt
        icon.source: receiptImage
        icon.height: 16
        icon.width: 16
        icon.color: CmnCfg.palette.iconMatte
        padding: 0
        background: Item {
        }
    }
}
