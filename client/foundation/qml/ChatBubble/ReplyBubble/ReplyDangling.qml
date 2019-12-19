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

Pane {
    id: replyWrapper

    background: ReplyBackground {
        clickEnabled: false
    }
    padding: CmnCfg.smallMargin

    contentHeight: unknownBody.height
    contentWidth: bubbleRoot.maxWidth * 0.8
    RowLayout {
        Text {
            id: unknownBody
            Layout.maximumWidth: bubbleRoot.maxWidth * 0.8
            font.family: CmnCfg.chatFont.name
            color: CmnCfg.palette.black
            textFormat: TextEdit.AutoText
            text: qsTr("Original message not found")
        }
    }
}
