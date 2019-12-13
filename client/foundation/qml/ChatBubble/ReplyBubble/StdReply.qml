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
        Layout.preferredWidth: replyElidedBody.width
                               > messageBody.width ? Math.min(
                                                         replyElidedBody.width,
                                                         bubbleRoot.maxWidth) : messageBody.width

        ReplyVerticalAccent {}
        ReplyMouseArea {}

        GridLayout {
            id: replyWrapperCol

            columns: 2
            rows: 3
            flow: GridLayout.TopToBottom

            ReplyLabel {
                Layout.alignment: Qt.AlignTop
            }

            ReplyElidedBody {
                id: replyElidedBody
            }

            ReplyTimeInfo {}

            Loader {
                id: imageClipLoader
                Layout.rowSpan: 3

                Component {
                    id: imageClipComponent
                    ReplyImageClip {}
                }
            }
        }
    }
}
