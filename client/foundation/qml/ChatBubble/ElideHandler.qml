import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0

// Collapse and expands message text if it is too long.
TextEdit {
    visible: elided
    text: !bubbleRoot.expanded ? qsTr("Read more") : qsTr("Collapse")
    font.bold: true
    font.family: CmnCfg.chatFont.name
    color: CmnCfg.palette.black
    Layout.leftMargin: CmnCfg.smallMargin
    Layout.rightMargin: CmnCfg.smallMargin
    Layout.bottomMargin: CmnCfg.smallMargin
    selectByMouse: false
    selectByKeyboard: false
    readOnly: true
    MouseArea {
        anchors.fill: parent
        onClicked: bubbleRoot.expanded = !bubbleRoot.expanded
    }

    TapHandler {
        onTapped: bubbleRoot.expanded = !bubbleRoot.expanded
    }
}
