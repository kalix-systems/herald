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

            width: bubbleRoot.maxWidth * 0.8 - imageClip.width
            ReplyFileClip {
                id: replyFileClip
                constraint: imageSize
            }

            ReplyFileSurplus {}

            ReplyElidedBody {
                id: replyElidedBody
                elideConstraint: imageSize
                maximumWidth: bubbleRoot.maxWidth * 0.8 - imageSize
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
