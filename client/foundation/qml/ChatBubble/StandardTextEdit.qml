import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0

TextEdit {
    text: body
    Layout.maximumWidth: maxWidth
    Layout.margins: CmnCfg.smallMargin / 2
    Layout.bottomMargin: 0
    wrapMode: Text.WrapAtWordBoundaryOrAnywhere
    Layout.alignment: Qt.AlignLeft
    selectByMouse: true
    selectByKeyboard: true
    readOnly: true
    font.family: CmnCfg.chatFont.name
    color: CmnCfg.palette.mainTextColor
}
