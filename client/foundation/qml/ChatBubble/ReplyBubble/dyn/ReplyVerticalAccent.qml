import QtQuick 2.14
import LibHerald 1.0

/// NOTE: Here be dragons, this depends on dynamic scoping.
/// Don't use this outside of the ReplyBubble directory
Rectangle {
    visible: knownReply
    anchors.right: !outbound ? replyWrapper.left : undefined
    anchors.left: outbound ? replyWrapper.right : undefined
    height: replyWrapper.height
    width: CmnCfg.smallMargin / 4
    color: opColor
}
