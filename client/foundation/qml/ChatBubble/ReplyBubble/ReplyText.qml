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

Page {
    id: replyWrapper

    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    messageModelData.opAuthor)]
    property string replyBody: messageModelData.opBody
    padding: CmnCfg.smallMargin

    background: ReplyBackground {}

    header: ReplyLabel {
        id: replyLabel
    }

    contentHeight: replyWrapperCol.height
    contentWidth: imageAttach ? 300 : ReplyWidthCalc.text(
                                    bubbleRoot.maxWidth,
                                    contentRoot.unameWidth, messageBody.width,
                                    contentRoot.messageStamps.width,
                                    replyLabel.opNameWidth,
                                    replyElidedBody.width, replyTimeInfo.width)

    Column {
        id: replyWrapperCol
        spacing: CmnCfg.smallMargin
        ReplyElidedBody {
            id: replyElidedBody
            maximumWidth: bubbleRoot.maxWidth // - CmnCfg.smallMargin * 2
        }

        ReplyTimeInfo {
            id: replyTimeInfo
        }
    }
}
