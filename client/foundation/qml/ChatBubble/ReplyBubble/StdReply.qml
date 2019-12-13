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

    spacing: 0

    Component.onCompleted: {
        if (modelData.opMediaAttachments.length === 0)
            return

        imageClipLoader.sourceComponent = imageClipComponent
        JS.parseMedia(modelData, imageClipLoader.item)
    }

    Rectangle {
        id: replyWrapper
        color: CmnCfg.palette.medGrey
        Layout.margins: CmnCfg.smallMargin
        Layout.preferredHeight: replyWrapperCol.height
        Layout.preferredWidth: replyWrapperCol.width

        ReplyVerticalAccent {}
        ReplyMouseArea {}

        ColumnLayout {
            id: replyWrapperCol

            RowLayout {
                id: replyRow
                height: reply.implicitHeight

                Layout.maximumWidth: bubbleRoot.imageAttach ? 300 : bubbleRoot.maxWidth
                Layout.minimumWidth: bubbleRoot.imageAttach ? 300 : messageBody.width
                clip: true

                ColumnLayout {
                    id: reply
                    spacing: 0

                    ReplyLabel {}

                    ReplyElidedBody {}

                    ReplyTimeInfo {}
                }

                Loader {
                    id: imageClipLoader

                    Component {
                        id: imageClipComponent
                        ReplyImageClip {}
                    }
                }
            }
        }
    }
}
