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

// TODO this should probably be called something to reflect that it's also used
// for contacts, not just conversations
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

    property color labelColor: CmnCfg.palette.black
    property color secondaryLabelColor: CmnCfg.palette.offBlack
    property int labelFontSize: CmnCfg.entityLabelSize
    property int subLabelFontSize: CmnCfg.entitySubLabelSize
    property alias bodyItalic: bodyText.font.italic

    // labeling constants
    GridLayout {
        id: labelGrid
        rows: 2
        columns: 2
        width: parent.width
        height: parent.height

        Label {
            id: name
            font {
                family: CmnCfg.chatFont.name
                pixelSize: labelFontSize
                weight: Font.Medium
            }
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
            Layout.preferredHeight: labelGrid.height * 0.25
            Layout.fillWidth: true
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
            Layout.preferredHeight: labelGrid.height * 0.25
            Layout.alignment: Qt.AlignRight | Qt.AlignTop
            color: secondaryLabelColor
        }

        Label {
            id: bodyText
            font {
                family: CmnCfg.chatFont.name
                pixelSize: subLabelFontSize
            }
            elide: "ElideRight"
            text: lastBody
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignLeft | Qt.alignTop
            Layout.maximumHeight: labelGrid.height * 0.25
            color: labelColor
            textFormat: Text.StyledText
        }

        Button {
            id: receiptImage
            icon.source: JS.receiptCodeSwitch(lastReceipt)
            icon.height: 24
            icon.width: 24
            icon.color: CmnCfg.palette.black
            padding: 0
            background: Item {}
        }
    }
}
