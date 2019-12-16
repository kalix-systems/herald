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

    color: CmnCfg.palette.medGrey

    height: unknownBody.height * 2
    width: ReplyWidthCalc.unknown(imageAttach, bubbleRoot.maxWidth,
                                  contentRoot.unameWidth,
                                  contentRoot.messageBody.width,
                                  unknownBody.width)

    StandardTextEdit {
        id: unknownBody
        maximumWidth: bubbleRoot.maxWidth
        text: qsTr("Original message not found")
    }
}
