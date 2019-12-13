import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../js/utils.mjs" as Utils
import "./js/utils.js" as JS
import QtQuick 2.13
import QtGraphicalEffects 1.12
import "../"
import "."
// Components that depend on dynamic scope
import "dyn"

ColumnLayout {
    id: wrapperCol

    // TODO move this into CmnCfg
    readonly property real imageSize: 80
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

        JS.parseMedia(modelData, imageClip)
    }

    Rectangle {
        id: replyWrapper
        color: CmnCfg.palette.medGrey
        Layout.margins: CmnCfg.smallMargin

        Layout.preferredHeight: replyWrapperCol.height
        Layout.preferredWidth: {
            // TODO move this and other complex layout calculations into Rust or C++
            if (imageAttach)
                return 300
            if (replyElidedBody.width > messageBody.width) {
                return Math.min(replyElidedBody.width,
                                bubbleRoot.maxWidth) + imageSize
            } else {
                const labelMax = Math.max(replyLabel.width, messageLabel.width)
                const bodyMax = Math.max(labelMax, messageBody.width)
                return bodyMax + CmnCfg.smallMargin * 2 + imageSize
            }
        }

        ReplyVerticalAccent {}
        ReplyMouseArea {}

        GridLayout {
            id: replyWrapperCol
            width: parent.width
            rows: 3
            flow: GridLayout.TopToBottom

            ReplyLabel {
                id: replyLabel
                Layout.alignment: Qt.AlignTop
            }

            ReplyElidedBody {
                id: replyElidedBody
                elideConstraint: imageClip.width
            }

            ReplyTimeInfo {}

            ReplyImageClip {
                id: imageClip
                Layout.rowSpan: 3
                Layout.alignment: Qt.AlignRight
            }
        }
    }
}
