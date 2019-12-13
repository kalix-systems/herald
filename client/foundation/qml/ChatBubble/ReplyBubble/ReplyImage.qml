import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../js/utils.mjs" as Utils
import "./js/utils.js" as JS
import QtQuick 2.13
import QtGraphicalEffects 1.12
import "../"
import "."
// Components that depend on dynamic scope
import "dyn"

Rectangle {
    id: replyWrapper

    // TODO move this into CmnCfg
    readonly property real imageSize: 80
    property real maxWidth: Math.min(parent.maxWidth, 600)
    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    messageModelData.opAuthor)]
    property bool knownReply: messageModelData.replyType === 2
    property string replyBody: knownReply ? messageModelData.opBody : ""

    Component.onCompleted: {
        if (messageModelData.opMediaAttachments.length === 0)
            return

        JS.parseMedia(messageModelData, imageClip)
    }

    color: CmnCfg.palette.medGrey

    height: Math.max(imageClip.height, replyWrapperCol.height)
    width: {
        // TODO move this and other complex layout calculations into Rust or C++
        if (imageAttach)
            return 300

        let out = 0

        if (replyElidedBody.width > messageBody.width) {
            out = replyWrapperCol.width
        } else {
            const labelMax = Math.max(replyLabel.width,
                                      contentRoot.messageLabel.width)
            const bodyMax = Math.max(labelMax, messageBody.width)
            out = bodyMax
        }

        return Math.min(bubbleRoot.maxWidth, out + imageClip.width)
    }

    ReplyMouseArea {}

    ReplyVerticalAccent {}

    Column {
        id: replyWrapperCol
        anchors.left: parent.left

        ReplyLabel {
            id: replyLabel
        }

        ReplyElidedBody {
            id: replyElidedBody
            elideConstraint: imageSize
            maximumWidth: bubbleRoot.maxWidth - imageSize
        }

        ReplyTimeInfo {}
    }

    ReplyImageClip {
        id: imageClip
        anchors.right: replyWrapper.right
    }
}
