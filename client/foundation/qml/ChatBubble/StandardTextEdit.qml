import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0

// This is a single cell layout
GridLayout {
    property real maximumWidth
    property alias text: _innerTextEdit.text

    TextEdit {
        id: _innerTextEdit
        Layout.maximumWidth: parent.maximumWidth
        text: if (bubbleRoot.elided && bubbleRoot.expanded) {
                  bubbleRoot.messageModelData.fullBody
              } else {
                  bubbleRoot.body
              }

        wrapMode: Text.WrapAtWordBoundaryOrAnywhere
        Layout.alignment: Qt.AlignLeft
        selectByMouse: true
        selectByKeyboard: true
        readOnly: true
        font.family: CmnCfg.chatFont.name
        font.pixelSize: CmnCfg.chatTextSize
        color: CmnCfg.palette.black
        textFormat: TextEdit.AutoText
        selectionColor: CmnCfg.palette.highlightColor

        MouseArea {
            anchors.fill: parent
            hoverEnabled: true
            onEntered: {
                if (bubbleRoot.moreInfo)
                    return
                bubbleRoot.hoverHighlight = true
            }
            onExited: {
                if (bubbleRoot.moreInfo)
                    return
                if (!bubbleRoot.hitbox.containsMouse) {
                    bubbleRoot.hoverHighlight = false
                }
            }
            propagateComposedEvents: true
            acceptedButtons: Qt.NoButton

            cursorShape: Qt.IBeamCursor
        }
    }
}
