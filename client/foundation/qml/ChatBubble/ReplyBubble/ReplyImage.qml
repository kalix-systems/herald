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
    property string replyBody: messageModelData.opBody
    property alias mouseEnabled: mouseArea.enabled
    property color opColor: CmnCfg.avatarColors[UserMap.get(
                                                    messageModelData.opAuthor).userColor]

    Component.onCompleted: JS.parseMedia(messageModelData, imageClip)

    color: CmnCfg.palette.medGrey
    width: bubbleRoot.maxWidth
    height: Math.max(wrapRow.height, imageClip.height)

    ReplyMouseArea {
        id: mouseArea
    }

    Rectangle {
        id: accent
        anchors.top: parent.top
        anchors.bottom: parent.bottom

        width: CmnCfg.accentBarWidth
        color: opColor
        anchors.left: parent.left
    }

    ReplyExpireInfo {
        id: expireInfo
        anchors.right: imageClip.left
    }

    Row {
        id: wrapRow
        spacing: CmnCfg.smallMargin
        anchors.left: accent.right
        topPadding: CmnCfg.smallMargin
        bottomPadding: CmnCfg.defaultMargin
        leftPadding: CmnCfg.smallMargin

        Item {
            //TODO: rename this, beware of dyn depending on this name.
            id: replyWrapperCol
            height: 64
            width: bubbleRoot.maxWidth - imageClip.width - CmnCfg.smallMargin * 3
            ReplyLabel {
                id: replyLabel
                anchors.top: parent.top
            }
            ReplyElidedBody {
                anchors.top: replyLabel.bottom
                anchors.topMargin: CmnCfg.smallMargin
                id: replyElidedBody
                elideConstraint: imageSize + bubbleRoot.maxWidth * 0.2
                maximumWidth: bubbleRoot.maxWidth * 0.8 - imageSize
                text: messageModelData.opBody
                      !== "" ? messageModelData.opBody : "<i>Media message</i>"
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
