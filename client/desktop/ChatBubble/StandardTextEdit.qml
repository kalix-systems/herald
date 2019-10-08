import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0

TextEdit {
    text: body
    Layout.maximumWidth: maxWidth
    Layout.margins: QmlCfg.smallMargin
    Layout.bottomMargin: QmlCfg.smallMargin
    wrapMode: Text.WrapAtWordBoundaryOrAnywhere
    Layout.alignment: Qt.AlignLeft
    selectByMouse: true
    selectByKeyboard: true
    readOnly: true
    color: outbound ? "black" : "white"
}
