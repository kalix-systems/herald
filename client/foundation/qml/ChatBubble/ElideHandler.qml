import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0

TextEdit {
    visible: elided
    text: "Read more"
    font.bold: true
    color: CmnCfg.palette.offBlack
    Layout.leftMargin: CmnCfg.smallMargin
    Layout.rightMargin: CmnCfg.smallMargin
    Layout.bottomMargin: CmnCfg.smallPadding
    selectByMouse: false
    selectByKeyboard: false
    readOnly: true
    MouseArea {
        anchors.fill: parent
        onClicked: {
            wrapperCol.expanded == true ? wrapperCol.expanded = false : wrapperCol.expanded = true
            parent.text == "Read more" ? parent.text = "Collapse" : parent.text = "Read more"
        }
    }
}
