import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Label {
    id: sender
    property string senderName
    property color senderColor
    text: senderName
    Layout.margins: CmnCfg.smallMargin / 2
    Layout.bottomMargin: CmnCfg.smallMargin / 2
    Layout.preferredHeight: CmnCfg.smallMargin
    font.bold: true
    color: senderColor
}
