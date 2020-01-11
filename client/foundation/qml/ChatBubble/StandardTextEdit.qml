import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0

// This is a single cell layout
GridLayout {
    property real maximumWidth
    property alias text: _innerTextEdit.text
    property bool isPreviewable: false
    TextEdit {
        id: _innerTextEdit
        Layout.maximumWidth: parent.maximumWidth
        text: if (bubbleRoot.elided && bubbleRoot.expanded) {
                  bubbleRoot.messageModelData.fullBody
              } else {
                  bubbleRoot.body
              }

        onLinkActivated: if (link[0] === "@") {
                             print("jump to the messages")
                         } else {
                             Qt.openUrlExternally(link)
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
            onEntered: bubbleActual.hoverHighlight = true
            onExited: if (!bubbleActual.hitbox.containsMouse) {
                          bubbleActual.hoverHighlight = false
                      }

            onClicked: mouse.accepted = false
            onPressAndHold: mouse.accepted = false
            onPressed: mouse.accepted = false
            onReleased: mouse.accepted = false
            propagateComposedEvents: true
            cursorShape: Qt.IBeamCursor
        }

        Component.onCompleted: {
            if (includes_link(text)) {
                text = generate_hyptertext(text)
            }
        }

        // Note about this regex:
        // it pulls every web URL out of the substring, it is used to replace
        // them with hypertext
        // capture group 2: the name of the website
        // capture group 3: the tld
        function includes_link(text) {
            const regexp = /\b(https?:\/\/)?([\w-\.]+)\.([a-zA-Z]{1,4})?(\/[\w-\/]*(\?\w*(=\w+)*[&\w-=]*)*((#|\.)[\w-]+)*)?/gmi
            return regexp.test(text)
        }

        function includes_message_ref(text) {}

        function generate_hyptertext(text) {
            const regexp = /\b(https?:\/\/)?([\w-\.]+)\.([a-zA-Z]{1,4})?(\/[\w-\/]*(\?\w*(=\w+)*[&\w-=]*)*((#|\.)[\w-]+)*)?/gmi
            return text.replace(regexp, replacer)
        }

        function replacer(match) {
            return "<a href=%1>%1</a>".arg(match)
        }
    }
}
