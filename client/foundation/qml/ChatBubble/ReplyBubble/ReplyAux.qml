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

Rectangle {
    id: replyWrapper

    property var auxData: JSON.parse(messageModelData.opAuxData)
    property string authorName: messageModelData.opName
    property string friendlyTimestamp: Utils.friendlyTimestamp(
                                           messageModelData.opInsertionTime)
    color: CmnCfg.palette.medGrey
    width: bubbleRoot.maxWidth
    height: replyWrapperCol.height
    property alias mouseEnabled: mouseArea.enabled

    ReplyMouseArea {
        id: mouseArea
    }

    Column {
        id: replyWrapperCol
        spacing: CmnCfg.smallMargin
        topPadding: CmnCfg.smallMargin
        bottomPadding: CmnCfg.defaultMargin
        leftPadding: CmnCfg.smallMargin

        padding: CmnCfg.smallMargin
        Text {
            text: friendlyTimestamp
            font.family: CmnCfg.chatFont.name
            font.italic: true
            font.pixelSize: 12
            color: CmnCfg.palette.darkGrey
            elide: Text.ElideRight
        }

        Text {
            id: actionText
            text: authorName + Utils.auxString(auxData.code, auxData.content)
            font.family: CmnCfg.chatFont.name
            font.italic: true
            elide: Text.ElideRight
        }

        Loader {
            id: imageClip
            active: auxData.code === 3
            height: active ? item.height : 0
            sourceComponent: ReplyImageClip {
                imageSource: "file:" + auxData.content
            }
        }
    }
}
