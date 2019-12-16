import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../js/utils.mjs" as Utils
import QtQuick 2.13
import QtGraphicalEffects 1.12
import "../"
import "./js/utils.js" as JS
// Components that depend on dynamic scope
import "dyn"

Page {
    id: replyWrapper

    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    messageModelData.opAuthor)]
    property string replyBody: messageModelData.opBody
    property int fileCount
    readonly property real imageSize: 80

    Component.onCompleted: {
        replyWrapper.fileCount = JS.parseDocs(replyFileClip.nameMetrics,
                                              messageModelData,
                                              replyFileClip.fileSize, fileCount)
        JS.parseMedia(messageModelData, imageClip)
    }

    padding: CmnCfg.smallMargin

    background: ReplyBackground {}

    header: ReplyLabel {
        id: replyLabel
    }
    contentHeight: wrapRow.implicitHeight
    contentWidth: wrapRow.implicitWidth

    Row {
        id: wrapRow
        spacing: CmnCfg.smallMargin

        Column {
            id: replyWrapperCol
            spacing: CmnCfg.smallMargin

            width: imageAttach ? (300 - imageClip.width) : ReplyWidthCalc.hybrid(
                                     bubbleRoot.maxWidth,
                                     contentRoot.unameWidth, messageBody.width,
                                     contentRoot.messageStamps.width,
                                     replyLabel.opNameWidth,
                                     replyElidedBody.width,
                                     replyTimeInfo.width, replyFileClip.width)
            //width: {
            //    if (imageAttach)
            //        return 300 - imageClip.width

            //    const rLabelWidth = replyLabel.opNameWidth
            //    const labelWidth = contentRoot.unameWidth

            //    const bodyWidth = messageBody.width
            //    const rBodyWidth = replyElidedBody.width

            //    const stampWidth = contentRoot.messageStamps.width
            //    const rTsWidth = replyTimeInfo.width

            //    const rWidth = Math.max(rLabelWidth, rBodyWidth, rTsWidth)
            //    const mWidth = Math.max(labelWidth, bodyWidth, stampWidth)

            //    const bubWidth = bubbleRoot.maxWidth - imageSize

            //    const docWidth = Math.max(150,
            //                              Math.min(bubWidth, Math.max(
            //                                           mWidth, rWidth,
            //                                           replyFileClip.width)))

            //    return docWidth
            //}
            ReplyFileClip {
                id: replyFileClip
                constraint: imageSize
            }

            ReplyFileSurplus {}

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
            anchors.top: replyWrapperCol.top
        }
    }
}
