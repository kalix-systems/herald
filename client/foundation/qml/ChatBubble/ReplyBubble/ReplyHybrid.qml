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

// TODO: describe each of these components with a comment above the first line
Rectangle {
    id: replyWrapper

    property color opColor: CmnCfg.avatarColors[UserMap.get(
                                                    messageModelData.opAuthor).userColor]
    property string replyBody: messageModelData.opBody
    property int fileCount
    readonly property real imageSize: 80
    property alias mouseEnabled: mouseArea.enabled

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

        width: CmnCfg.accentBarWidth
        color: opColor
        anchors.left: parent.left
    }

    ReplyMouseArea {
        id: mouseArea
    }

    height: wrapRow.height
    width: bubbleRoot.maxWidth
    color: CmnCfg.palette.medGrey

    ReplyExpireInfo {
        anchors.right: imageClip.left
    }

    Row {
        id: wrapRow
        spacing: CmnCfg.smallMargin
        anchors.left: accent.right
        topPadding: CmnCfg.smallMargin
        bottomPadding: CmnCfg.defaultMargin
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
                elideWidth: bubbleRoot.maxWidth
            }

            ReplyFileSurplus {}

            ReplyElidedBody {
                id: replyElidedBody

                maximumWidth: bubbleRoot.maxWidth * 0.8 - imageSize
            }
        }
    }
    ReplyImageClip {
        id: imageClip
        anchors.top: parent.top
        anchors.right: parent.right
        clipSize: wrapRow.height
    }
}
