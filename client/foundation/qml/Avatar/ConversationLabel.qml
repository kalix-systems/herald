import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as JS

// TODO:
// there are some loose magic numbers
// hanging around in the font sizes. move those to CmnCfg
// TODO:
// move the property translation functions into
// some common js directory , receipt urls are not numbers, nor are timestamps
Item {
    // the group name or displayName of the conversation
    property string contactName
    // the previous message of the conversation, or the empty string
    property string lastBody
    // the previous latest human readable timestamp, or the empty string
    property string lastTimestamp
    // the value of the latest read receipt according to the ReceiptStatus enum
    property int lastReceipt: 0
    property string lastAuthor
    property color labelColor
    property int labelSize

    // labeling constants
    GridLayout {
        id: labelGrid
        rows: 2
        columns: 2
        width: parent.width
        height: parent.height
        Label {
            id: uid
            font {
                bold: true
                family: CmnCfg.chatFont.name
                pixelSize: labelSize
            }
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
            Layout.preferredHeight: labelGrid.height / 4
            Layout.maximumWidth: parent.width
            elide: "ElideRight"
            text: contactName
            color: labelColor
        }

        Label {
            id: ts
            font {
                family: CmnCfg.chatFont.name
                pixelSize: 11
            }
            text: lastTimestamp
            Layout.preferredHeight: labelGrid.height / 4
            Layout.alignment: Qt.AlignRight | Qt.AlignTop
            color: CmnCfg.palette.secondaryColor
        }

        Label {
            id: bodyText
            font {
                family: CmnCfg.chatFont.name
                pixelSize: 13
            }
            elide: "ElideRight"
            text: lastBody
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignLeft | Qt.alignTop
            Layout.maximumHeight: labelGrid.height / 2
            color: CmnCfg.palette.secondaryColor
        }

        Button {
            id: receiptImage
            icon.source: JS.receiptCodeSwitch(lastReceipt)
            icon.height: 24
            icon.width: 24
            icon.color: CmnCfg.palette.mainTextColor
            padding: 0
            background: Item {
            }
        }
    }
}
