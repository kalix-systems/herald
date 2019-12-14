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
    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    messageModelData.opAuthor)]
    property string replyBody: messageModelData.opBody

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

        const rLabelWidth = replyLabel.opNameWidth
        const labelWidth = contentRoot.unameWidth

        const bodyWidth = messageBody.width
        const rBodyWidth = replyElidedBody.width

        const stampWidth = contentRoot.messageStamps.width
        const rTsWidth = replyTimeInfo.width

        const rWidth = Math.max(rLabelWidth, rBodyWidth, rTsWidth)
        const mWidth = Math.max(labelWidth, bodyWidth, stampWidth)

        const bubWidth = bubbleRoot.maxWidth

        if ((mWidth - rWidth) < 80) {
            return Math.min(bubWidth, rWidth + imageClip.width)
        } else {
            return Math.min(bubWidth, mWidth)
        }
    }

    ReplyMouseArea {}

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

        ReplyTimeInfo {
            id: replyTimeInfo
        }
    }

    ReplyImageClip {
        id: imageClip
        anchors.right: replyWrapper.right
    }
}
