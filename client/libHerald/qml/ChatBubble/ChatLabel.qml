import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Label {
    id: sender
    property string senderName
    readonly property bool emptyName: senderName === ""
    text: senderName
    Layout.margins: emptyName ? 0 : QmlCfg.smallMargin
    Layout.bottomMargin: emptyName ? QmlCfg.smallMargin : QmlCfg.margin
    Layout.preferredHeight: !emptyName ? QmlCfg.margin : 0
    font.bold: true
    color: QmlCfg.palette.mainTextColor
}
