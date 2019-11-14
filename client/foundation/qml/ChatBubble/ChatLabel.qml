import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import QtQuick.Layouts 1.12

Label {
    id: sender
    property string senderName
    property color senderColor
    text: senderName
    color: senderColor
    Layout.leftMargin: CmnCfg.smallMargin
    Layout.rightMargin: CmnCfg.smallMargin
    Layout.bottomMargin: CmnCfg.margin / 2
    Layout.topMargin: CmnCfg.margin / 2
    Layout.preferredHeight: CmnCfg.smallMargin
  font.bold: true
}
