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

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    messageModelData.opAuthor)]
    property var replyId
    property bool knownReply: messageModelData.replyType === 2
    property string replyBody: knownReply ? messageModelData.opBody : ""

    color: CmnCfg.palette.medGrey

    height: replyWrapperCol.height
    width: {
        // TODO move this and other complex layout calculations into Rust or C++
        if (imageAttach)
            return 300

        const labelMax = Math.max(replyLabel.width,
                                  contentRoot.messageLabel.width)

        const bodyMax = Math.max(labelMax, contentRoot.messageBody.width)

        if (replyElidedBody.width > contentRoot.messageBody.width) {
            return Math.min(Math.max(replyElidedBody.width, bodyMax),
                            bubbleRoot.maxWidth)
        } else {
            return bodyMax
        }
    }

    ReplyMouseArea {}

    ReplyVerticalAccent {
        id: replyVerticalAccent
    }

    Column {
        id: replyWrapperCol

        ReplyLabel {
            id: replyLabel
        }

        ReplyElidedBody {
            id: replyElidedBody
            maximumWidth: bubbleRoot.maxWidth
        }

        ReplyTimeInfo {}
    }
}
