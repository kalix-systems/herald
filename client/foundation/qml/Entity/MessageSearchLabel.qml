import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as JS

Item {
    // the group name or displayName of the conversation
    property string conversationTitle
    // text before the first search term match in this label body
    property string beforeMatch
    // the first search term match in this label body
    property string match
    // text after the first search term match in this label body
    property string afterMatch
    // the human readable timestamp of the matched message
    property string timestamp

    property color labelColor: CmnCfg.palette.black
    property color secondaryLabelColor: CmnCfg.palette.offBlack
    property int labelFontSize: CmnCfg.entityLabelSize
    property int subLabelFontSize: CmnCfg.entitySubLabelSize

    GridLayout {
        id: labelGrid
        rows: bodyText.lineCount > 1 ? 3 : 2
        columns: 2
        width: parent.width

        Label {
            id: title
            font {
                weight: Font.Medium
                family: CmnCfg.chatFont.name
                // TODO change when we make font defaults make sense
                pixelSize: labelFontSize
            }

            Layout.topMargin: labelGrid.rows > 2 ? - 6 : 0
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
            Layout.fillWidth: true
            elide: "ElideRight"
            text: messageData.conversationTitle
            color: messageRectangle.state
                   !== "" ? CmnCfg.palette.black : CmnCfg.palette.lightGrey
        }

        Label {
            id: ts
            font {
                family: CmnCfg.chatFont.name
                //TODO: Magic number erasure, we need a secondary small label size
                pixelSize: 11
            }
            text: timestamp
            Layout.preferredHeight: ts.height
            Layout.alignment: Qt.AlignRight | Qt.AlignTop
            color: messageRectangle.state
                   !== "" ? CmnCfg.palette.offBlack : CmnCfg.palette.medGrey
        }

        TextMetrics {
            id: prefix
            text: beforeMatch
            elide: Text.ElideLeft
            elideWidth: labelGrid.width * 2
        }

        Label {
            id: bodyText
            font {
                family: CmnCfg.chatFont.name
                pixelSize: subLabelFontSize
            }

            Layout.topMargin: labelGrid.rows > 2 ? -CmnCfg.smallMargin : 0
            elide: "ElideRight"
            text: if (beforeMatch.length === 0) {
                      match + afterMatch
                  } else if (prefix.length === beforeMatch.length) {
                      prefix.elidedText + match + afterMatch
                  } else {
                      "..." + prefix.elidedText + match + afterMatch
                  }

            Layout.fillWidth: true
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
            color: labelColor
            textFormat: Text.StyledText
            wrapMode: Text.WrapAtWordBoundaryOrAnywhere
        }
    }
}
