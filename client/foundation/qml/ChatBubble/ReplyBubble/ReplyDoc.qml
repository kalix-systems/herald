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

ColumnLayout {
    id: wrapperCol

    property real maxWidth: Math.min(parent.maxWidth, 600)
    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    modelData.opAuthor)]
    property var replyId
    property bool knownReply: modelData.replyType === 2
    property string replyBody: knownReply ? modelData.opBody : ""
    property var modelData
    property string fileCount

    spacing: 0

    Component.onCompleted: JS.parseDocs(replyFileClip.nameMetrics, modelData,
                                        replyFileClip.fileSize, fileCount)

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: replyWrapperCol.height
        color: CmnCfg.palette.medGrey
        Layout.margins: CmnCfg.smallMargin
        Layout.minimumWidth: 150
        Layout.preferredWidth: replyWrapperCol.width

        ReplyVerticalAccent {}

        ReplyMouseArea {}

        // wraps op label + op doc clip + op message body
        ColumnLayout {
            id: replyWrapperCol

            ReplyLabel {}

            // wraps op file clip
            ReplyFileClip {
                id: replyFileClip
            }

            // file +n count
            ReplyFileSurplus {}

            // op message body
            ColumnLayout {
                id: reply
                spacing: 0
                Layout.maximumWidth: bubbleRoot.imageAttach ? 300 : bubbleRoot.maxWidth
                Layout.minimumWidth: bubbleRoot.imageAttach ? 300 : Math.max(
                                                                  300,
                                                                  messageBody.width)

                ReplyElidedBody {}

                ReplyTimeInfo {}
            }
        }
    }
}
