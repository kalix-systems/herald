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
    property color opColor: CmnCfg.avatarColors[UserMap.get(
                                                    messageModelData.opAuthor).userColor]
    property string replyBody: messageModelData.opBody
    property int fileCount

    Component.onCompleted: replyWrapper.fileCount = JS.parseDocs(
                               replyFileClip.nameMetrics, messageModelData,
                               replyFileClip.fileSize, replyWrapper)

    height: replyWrapperCol.height
    width: bubbleRoot.maxWidth
    color: CmnCfg.palette.medGrey
    property alias mouseEnabled: mouseArea.enabled

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

    ReplyExpireInfo {}
    Column {
        id: replyWrapperCol
        spacing: CmnCfg.smallMargin
        anchors.left: accent.right
        topPadding: CmnCfg.smallMargin
        bottomPadding: CmnCfg.defaultMargin
        leftPadding: CmnCfg.smallMargin

        ReplyLabel {
            id: replyLabel
        }

        ReplyFileClip {
            id: replyFileClip
            elideWidth: bubbleRoot.maxWidth
        }

        ReplyFileSurplus {}
        ReplyElidedBody {
            id: replyElidedBody
            maximumWidth: bubbleRoot.maxWidth
        }
    }
}
