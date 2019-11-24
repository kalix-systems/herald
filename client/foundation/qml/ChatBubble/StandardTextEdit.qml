import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0

TextEdit {
    text: body
    Layout.maximumWidth: maxWidth
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
    color: CmnCfg.palette.mainTextColor
    textFormat: TextEdit.RichText
}
