import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Label {
    id: sender
    property string senderName
    readonly property bool emptyName: senderName === ""
    text: senderName
    Layout.margins: emptyName ? 0 : CmnCfg.smallMargin
    Layout.bottomMargin: emptyName ? CmnCfg.smallMargin : CmnCfg.margin
    Layout.preferredHeight: !emptyName ? CmnCfg.margin : 0
    font.bold: true
    color: CmnCfg.palette.mainTextColor
}
