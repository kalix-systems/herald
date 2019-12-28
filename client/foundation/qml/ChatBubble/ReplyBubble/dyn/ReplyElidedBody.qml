import QtQuick 2.14
import QtQuick.Layouts 1.14
import "../.."
import LibHerald 1.0

// NOTE: Here be dragons: this relies on dynamic scoping
/// Don't use this outside of the ReplyBubble directory
GridLayout {
    property real maximumWidth
    property real elideConstraint: 0

    Text {
        id: _innerTextEdit
        Layout.maximumWidth: parent.maximumWidth
        text: opBodyTextMetrics.elidedText

        wrapMode: Text.WrapAtWordBoundaryOrAnywhere

        Layout.alignment: Qt.AlignLeft
        font.family: CmnCfg.chatFont.name
        color: CmnCfg.palette.black
        textFormat: TextEdit.AutoText

        TextMetrics {
            id: opBodyTextMetrics
            text: messageModelData.opBody
            elideWidth: (bubbleRoot.maxWidth - elideConstraint) * 2
            elide: Text.ElideRight
        }
    }
}
