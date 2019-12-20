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

Rectangle {
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

    Rectangle {
        id: accent
        anchors.top: parent.top
        anchors.bottom: parent.bottom

        width: CmnCfg.smallMargin / 2
        color: opColor
        anchors.left: parent.left
    }

    ReplyMouseArea {}

    height: wrapRow.height
    width: bubbleRoot.maxWidth
    color: CmnCfg.palette.medGrey

    Row {
        id: wrapRow
        spacing: CmnCfg.smallMargin
        anchors.left: accent.right
        topPadding: CmnCfg.smallMargin
        bottomPadding: CmnCfg.margin
        leftPadding: CmnCfg.smallMargin

        Column {
            id: replyWrapperCol
            ReplyLabel {
                id: replyLabel
            }
            spacing: CmnCfg.smallMargin

            width: bubbleRoot.maxWidth - imageClip.width - CmnCfg.smallMargin * 3
            ReplyFileClip {
                id: replyFileClip
                constraint: imageSize
            }

            ReplyFileSurplus {}

            ReplyElidedBody {
                id: replyElidedBody

                maximumWidth: bubbleRoot.maxWidth * 0.8 - imageSize
            }
        }

        ReplyImageClip {
            id: imageClip
            anchors.top: replyWrapperCol.top
        }
    }
}
