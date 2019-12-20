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

Rectangle {
    id: replyWrapper

    // TODO move this into CmnCfg
    readonly property real imageSize: 80
    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    messageModelData.opAuthor)]
    property string replyBody: messageModelData.opBody

    Component.onCompleted: JS.parseMedia(messageModelData, imageClip)

    color: CmnCfg.palette.medGrey
    width: bubbleRoot.maxWidth
    height: wrapRow.height

    ReplyMouseArea {}

    Rectangle {
        id: accent
        anchors.top: parent.top
        anchors.bottom: parent.bottom

        width: CmnCfg.smallMargin / 2
        color: opColor
        anchors.left: parent.left
    }

    Row {
        id: wrapRow
        spacing: CmnCfg.smallMargin
        anchors.left: accent.right
        topPadding: CmnCfg.smallMargin
        bottomPadding: CmnCfg.margin
        leftPadding: CmnCfg.smallMargin

        Item {
            id: replyWrapperCol
            height: 64
            width: bubbleRoot.maxWidth * 0.8
            ReplyLabel {
                id: replyLabel
                anchors.top: parent.top
            }
            ReplyElidedBody {
                anchors.top: replyLabel.bottom
                anchors.topMargin: CmnCfg.smallMargin
                id: replyElidedBody
                elideConstraint: imageSize
                maximumWidth: bubbleRoot.maxWidth * 0.8 - imageSize
            }
        }

        ReplyImageClip {
            id: imageClip
            anchors.top: replyWrapperCol.top
        }
    }
}
