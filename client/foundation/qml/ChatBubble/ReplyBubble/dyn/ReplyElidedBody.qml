import QtQuick 2.14
import QtQuick.Layouts 1.14
import "../.."
import LibHerald 1.0

// NOTE: Here be dragons: this relies on dynamic scoping
/// Don't use this outside of the ReplyBubble directory
StandardTextEdit {
    text: opBodyTextMetrics.elidedText
    property real elideConstraint: 0
    Layout.topMargin: 0
    Layout.rightMargin: 0
    Layout.bottomMargin: 0
    Layout.leftMargin: 0
    Layout.maximumWidth: bubbleRoot.maxWidth

    TextMetrics {
        id: opBodyTextMetrics
        property string shortenedText: knownReply ? modelData.opBody : qsTr(
                                                        "Original message not found")
        text: shortenedText
        elideWidth: (bubbleRoot.maxWidth - elideConstraint) * 2
        elide: Text.ElideRight
    }
}
