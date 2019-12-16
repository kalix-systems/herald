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

    color: CmnCfg.palette.white
    border.color: CmnCfg.palette.black
    border.width: 1
    anchors.margins: CmnCfg.smallMargin

    height: unknownBody.height * 2
    width: {
        // TODO move this and other complex layout calculations into Rust or C++
        if (imageAttach)
            return 300

        const bodyMax = Math.max(contentRoot.unameWidth, messageBody.width)

        if (unknownBody.width > messageBody.width) {
            return Math.min(Math.max(unknownBody.width, bodyMax),
                            bubbleRoot.maxWidth) + CmnCfg.smallMargin * 2
        } else {
            return bodyMax + CmnCfg.smallMargin * 2
        }
    }

    StandardTextEdit {
        id: unknownBody
        rowSpacing: CmnCfg.smallMargin
        columnSpacing: CmnCfg.smallMargin
        maximumWidth: bubbleRoot.maxWidth
        text: qsTr("Original message not found")
    }
}
