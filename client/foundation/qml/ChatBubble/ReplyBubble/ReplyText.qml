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

    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    messageModelData.opAuthor)]
    property string replyBody: messageModelData.opBody

    color: CmnCfg.palette.medGrey

    height: replyWrapperCol.height
    width: {
        // TODO move this and other complex layout calculations into Rust or C++
        if (imageAttach)
            return 300

        const rLabelWidth = replyLabel.opNameWidth
        const labelWidth = contentRoot.unameWidth

        const bodyWidth = messageBody.width
        const rBodyWidth = replyElidedBody.width

        const stampWidth = contentRoot.messageStamps.width
        const rTsWidth = replyTimeInfo.width

        const rWidth = Math.max(rLabelWidth, rBodyWidth, rTsWidth)
        const mWidth = Math.max(labelWidth, bodyWidth, stampWidth)

        const bubWidth = bubbleRoot.maxWidth

        return Math.min(bubWidth, Math.max(rWidth, mWidth))
    }

    ReplyMouseArea {}

    Column {
        id: replyWrapperCol

        ReplyLabel {
            id: replyLabel
        }

        ReplyElidedBody {
            id: replyElidedBody
            maximumWidth: bubbleRoot.maxWidth
        }

        ReplyTimeInfo {
            id: replyTimeInfo
        }
    }
}
