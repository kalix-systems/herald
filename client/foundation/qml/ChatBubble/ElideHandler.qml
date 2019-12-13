import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0

TextEdit {
    visible: elided
    text: !!contentRoot.expanded ? qsTr("Read more") : qsTr("Collapse")
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
        onClicked: contentRoot.expanded = !contentRoot.expanded
    }

    TapHandler {
        onTapped: contentRoot.expanded = !contentRoot.expanded
    }
}
