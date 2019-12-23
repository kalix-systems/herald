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
    property color opColor: CmnCfg.avatarColors[messageModelData.opColor]

    property string replyBody: messageModelData.opBody
    property int fileCount

    Component.onCompleted: replyWrapper.fileCount = JS.parseDocs(
                               replyFileClip.nameMetrics, messageModelData,
                               replyFileClip.fileSize, replyWrapper)

    padding: CmnCfg.smallMargin

    background: ReplyBackground {}

    header: ReplyLabel {
        id: replyLabel
    }

    contentHeight: replyWrapperCol.implicitHeight
    contentWidth: bubbleRoot.maxWidth * 0.8
    Column {
        id: replyWrapperCol
        spacing: CmnCfg.smallMargin

        ReplyFileClip {
            id: replyFileClip
        }

        ReplyFileSurplus {}
        ReplyElidedBody {
            id: replyElidedBody
            maximumWidth: bubbleRoot.maxWidth
        }

        ReplyTimeInfo {
            id: replyTimeInfo
        }
    }
}
