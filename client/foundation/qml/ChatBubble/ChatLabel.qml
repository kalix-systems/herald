import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Label {
    id: sender
    property string senderName
    property color senderColor
    text: senderName
    Layout.margins: CmnCfg.smallMargin
    Layout.bottomMargin: CmnCfg.margin
    Layout.preferredHeight: CmnCfg.margin
    font.bold: true
    color: senderColor
}
