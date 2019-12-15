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

Page {
    id: replyWrapper

    // TODO move this into CmnCfg
    readonly property real imageSize: 80
    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    messageModelData.opAuthor)]
    property string replyBody: messageModelData.opBody

    Component.onCompleted: JS.parseMedia(messageModelData, imageClip)

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

        Item {
            id: replyWrapperCol
            height: 64
            width: {
                if (imageAttach)
                    return 300 - imageClip.width

                const rLabelWidth = replyLabel.opNameWidth
                const labelWidth = contentRoot.unameWidth

                const bodyWidth = messageBody.width
                const rBodyWidth = replyElidedBody.width

                const stampWidth = contentRoot.messageStamps.width
                const rTsWidth = replyTimeInfo.width

                const rWidth = Math.max(rLabelWidth, rBodyWidth, rTsWidth)
                const mWidth = Math.max(labelWidth, bodyWidth, stampWidth)

                const bubWidth = bubbleRoot.maxWidth - imageSize

                return Math.min(bubWidth, Math.max(rWidth, mWidth))
            }

            ReplyElidedBody {
                anchors.top: parent.top
                id: replyElidedBody
                elideConstraint: imageSize
                maximumWidth: bubbleRoot.maxWidth - imageSize
            }

            ReplyTimeInfo {
                anchors.bottom: parent.bottom
                id: replyTimeInfo
            }

            // Debug component
            //Rectangle {
            //    anchors.fill: parent
            //    opacity: 0.5
            //    color: "blue"
            //    border.color: "black"
            //    border.width: 1
            //}
        }

        ReplyImageClip {
            id: imageClip
            anchors.top: replyWrapperCol.top
        }
    }
}
