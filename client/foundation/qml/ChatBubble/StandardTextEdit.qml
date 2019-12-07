import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0

TextEdit {
    text: if (parent.elided && parent.expanded) {
              fullBody
          } else if (parent.elided) {
              body + "..."
          } else {
              body
          }

    Layout.topMargin: CmnCfg.margin / 2
    Layout.leftMargin: CmnCfg.smallMargin
    Layout.rightMargin: CmnCfg.smallMargin
    Layout.bottomMargin: CmnCfg.smallPadding

    wrapMode: Text.WrapAtWordBoundaryOrAnywhere
    Layout.alignment: Qt.AlignLeft
    selectByMouse: true
    selectByKeyboard: true
    readOnly: true
    font.family: CmnCfg.chatFont.name
    color: CmnCfg.palette.black
    textFormat: TextEdit.AutoText
    selectionColor: CmnCfg.palette.highlightColor
}
