import "../js/utils.js" as JS
import QtQuick 2.14
import LibHerald 1.0

// NOTE: Here be dragons: this relies on dynamic scoping
// Don't use this outside of the ReplyBubble directory
MouseArea {
    anchors.fill: parent
    z: CmnCfg.overlayZ
    onClicked: JS.jumpHandler(replyId, ownedConversation, convWindow)
}
