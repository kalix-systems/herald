import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../js/utils.mjs" as Utils
import "./js/utils.js" as JS
import QtQuick 2.13
import QtGraphicalEffects 1.12
import "../"
// Components that depend on dynamic scope
import "dyn"

Rectangle {
    id: replyWrapper

    color: CmnCfg.palette.offBlack

    height: wrapperRow.height
    width: bubbleRoot.maxWidth
    Row {
        id: wrapperRow
        padding: CmnCfg.smallMargin
        Text {
            id: unknownBody
            width: bubbleRoot.maxWidth * 0.8
            font.family: CmnCfg.chatFont.name
            color: CmnCfg.palette.white
            textFormat: TextEdit.AutoText
            text: qsTr("REDACTED")
        }
    }
}
