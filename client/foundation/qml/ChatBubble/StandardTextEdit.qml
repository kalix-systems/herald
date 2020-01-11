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

        onLinkActivated: if (false) {

                         } else if (false) {

                         }

        function includes_link(text) {
            var regexp = "@(https?|ftp):\/\/[^\s/$.?#].[^\s]*$@iS*"
            return regexp.test(text)
        }

        function includes_message_ref(text) {}

        function generate_hyptertext(text) {
            var regexp = "@(https?|ftp):\/\/[^\s/$.?#].[^\s]*$@iS*"
            return text.replace(regexp, replacer)
        }

        function replacer(match) {
            print(match)
            return "grappo"
        }
    }
}
