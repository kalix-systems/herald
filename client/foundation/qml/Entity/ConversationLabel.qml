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
    property string convoTitle

    property color labelColor: CmnCfg.palette.black
    property color minorTextColor: CmnCfg.palette.offBlack
    property int labelFontSize: CmnCfg.entityLabelSize
    property int subLabelFontSize: CmnCfg.entitySubLabelSize
    property alias bodyItalic: bodyText.font.italic

    // json summary
    property string lastMsgDigest

    // This component expects one of the following groups of properties,
    // either a lastMsgDigest property (a JSON object), or the subsequent group of
    // properties calculated from it.

    // OPTION 1: lastMsgDigest
    // the bundle this label represents.
    property var lastMessage: lastMsgDigest !== "" ? JSON.parse(
                                                         lastMsgDigest) : undefined
    property bool isEmpty: true

    readonly property bool __init: !isEmpty && lastMessage !== undefined

    // OPTION 2: lastReceipt, outbound, lastAuthor, lastTimestamp, and lastBody
    // the value of the latest read receipt according to the ReceiptStatus enum
    property int lastReceipt: 0
    // true if the last message was sent by the logged-in user
    property bool outbound: !__init ? false : lastMessage.author === Herald.config.configId
    // user who sent the last message in the conversation
    property string lastAuthor: {
        if (!__init) {
            return ""
        }

        if (outbound)
            return qsTr('You')

        if (!isEmpty)
            return Herald.users.nameById(lastMessage.author)

        return ''
    }
    // the previous latest human readable timestamp, or the empty string
    property string lastTimestamp: __init ? JS.friendlyTimestamp(
                                                lastMessage.time) : ""

    // the previous message of the conversation, or the empty string
    property string lastBody: {
        if (!__init)
            return ""

        if (lastMessage.auxCode !== null) {
            return "<i>" + lastAuthor + JS.auxStringShort(
                        lastMessage.auxCode) + "</i>"
        }

        if (lastMessage.body === null && lastMessage.hasAttachments) {
            return lastAuthor + ": " + "<i>Media message</i>"
        }

        if (lastMessage.body !== null) {
            return lastAuthor + ": " + lastMessage.body
        }

        return ''
    }

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
            text: convoTitle
            color: labelColor
        }

        Label {
            id: ts
            font {
                family: CmnCfg.chatFont.name
                pixelSize: CmnCfg.minorTextSize
            }
            text: lastTimestamp
            Layout.preferredHeight: labelGrid.height * 0.25
            Layout.alignment: Qt.AlignRight | Qt.AlignTop
            color: minorTextColor
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
            icon.height: 20
            icon.width: 20
            icon.color: CmnCfg.palette.iconFill
            padding: 0
            background: Item {}
        }
    }
}
