import QtQuick 2.12
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

    Component.onCompleted: {
        JS.parseDocs(replyFileClip.nameMetrics, modelData,
                     replyFileClip.fileSize, fileCount)
        JS.parseMedia(modelData, imageClip)
    }

    Rectangle {
        id: replyWrapper
        Layout.preferredHeight: replyWrapperCol.height
        color: CmnCfg.palette.medGrey
        Layout.margins: CmnCfg.smallMargin
        Layout.minimumWidth: 150
        Layout.preferredWidth: replyWrapperCol.width

        ReplyVerticalAccent {}

        ReplyMouseArea {}

        //wraps attachment content and op message body
        ColumnLayout {
            id: replyWrapperCol
            spacing: 0

            //wraps op label + op doc clip + op media clip
            Item {
                Layout.preferredWidth: reply.width
                Layout.preferredHeight: 80

                //wraps op label + op doc clip
                ColumnLayout {
                    id: fileLabelWrapper
                    anchors.left: parent.left

                    ReplyLabel {}

                    //wraps doc clip
                    ReplyFileClip {
                        id: replyFileClip
                    }
                    //+ n more file count
                    ReplyFileSurplus {}
                }

                //op image clip
                ReplyImageClip {
                    id: imageClip
                    anchors.topMargin: CmnCfg.smallMargin
                    anchors.top: parent.top
                    anchors.right: parent.right
                }
            }

            //op message body + timestamp
            ColumnLayout {
                id: reply
                spacing: 0
                Layout.topMargin: 0
                Layout.rightMargin: CmnCfg.smallMargin
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
