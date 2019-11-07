import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0

Label {
    id: sender
    property string senderName
    property color senderColor
    text: senderName
    color: senderColor
    font.family: CmnCfg.chatFont.name
}
