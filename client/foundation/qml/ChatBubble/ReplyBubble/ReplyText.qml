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
    property color opColor: CmnCfg.avatarColors[messageModelData.opColor]

    property string replyBody: messageModelData.opBody
    color: CmnCfg.palette.medGrey
    width: bubbleRoot.maxWidth
    height: replyWrapperCol.height

    ReplyMouseArea {}

    Rectangle {
        id: accent
        anchors.top: parent.top
        anchors.bottom: parent.bottom

        width: CmnCfg.smallMargin / 2
        color: opColor
        anchors.left: parent.left
    }

    Column {
        id: replyWrapperCol
        spacing: CmnCfg.smallMargin
        topPadding: CmnCfg.smallMargin
        bottomPadding: CmnCfg.margin
        leftPadding: CmnCfg.smallMargin
        anchors.left: accent.right

        ReplyLabel {
            id: replyLabel
        }

        ReplyElidedBody {
            id: replyElidedBody
            maximumWidth: bubbleRoot.maxWidth - CmnCfg.smallMargin * 3 // - CmnCfg.smallMargin * 2
        }
    }
}
