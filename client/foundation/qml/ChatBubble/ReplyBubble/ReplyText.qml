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

Page {
    id: replyWrapper

    property color opColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                    messageModelData.opAuthor)]
    property string replyBody: messageModelData.opBody

    height: implicitHeaderHeight + replyWrapperCol.height
    background: Rectangle {
        color: CmnCfg.palette.lightGrey
        anchors.fill: parent
        border.color: "black"
        border.width: 1
        ReplyMouseArea {}
    }

    header: Label {
        id: replyLabel
        readonly property real opNameWidth: opNameTM.width
        text: opNameTM.text
        font.weight: Font.Bold
        font.family: CmnCfg.chatFont.name

        padding: CmnCfg.smallMargin / 2
        color: CmnCfg.palette.white
        horizontalAlignment: Text.AlignLeft

        background: Rectangle {
            color: opColor
            border.color: Qt.darker(color, 1.5)
            border.width: 1
        }

        TextMetrics {
            id: opNameTM
            text: Herald.users.nameById(messageModelData.opAuthor)
        }
    }

    // height: replyWrapperCol.height
    width: {
        // TODO move this and other complex layout calculations into Rust or C++
        if (imageAttach)
            return 300

        const rLabelWidth = replyLabel.opNameWidth
        const labelWidth = contentRoot.unameWidth

        const bodyWidth = messageBody.width
        const rBodyWidth = replyElidedBody.width

        const stampWidth = contentRoot.messageStamps.width
        const rTsWidth = replyTimeInfo.width

        const rWidth = Math.max(rLabelWidth, rBodyWidth, rTsWidth)
        const mWidth = Math.max(labelWidth, bodyWidth, stampWidth)

        const bubWidth = bubbleRoot.maxWidth

        return Math.min(bubWidth, Math.max(rWidth,
                                           mWidth) + CmnCfg.smallMargin * 2)
    }

    Column {
        id: replyWrapperCol
        spacing: CmnCfg.smallMargin
        padding: CmnCfg.smallMargin

        //        ReplyLabel {
        //            id: replyLabel
        //        }
        ReplyElidedBody {
            id: replyElidedBody
            maximumWidth: bubbleRoot.maxWidth - CmnCfg.smallMargin * 2
        }

        ReplyTimeInfo {
            id: replyTimeInfo
        }
    }
}
