import QtQuick 2.14
import QtQuick.Layouts 1.14
import "../.."
import LibHerald 1.0

// NOTE: Here be dragons: this relies on dynamic scoping
/// Don't use this outside of the ReplyBubble directory
StandardTextEdit {
    text: opBodyTextMetrics.elidedText
    property real elideConstraint: 0

    TextMetrics {
        id: opBodyTextMetrics
        property string shortenedText: knownReply ? messageModelData.opBody : qsTr(
                                                        "Original message not found")
        text: shortenedText
        elideWidth: (bubbleRoot.maxWidth - elideConstraint) * 2
        elide: Text.ElideRight
    }
}
