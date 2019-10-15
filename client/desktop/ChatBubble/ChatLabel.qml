import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Label {
    property string senderName
    id: sender
    // BNOTE: This boolean should be a property:w
    text: senderName === "" ? "" : senderName
    Layout.margins: senderName === "" ? 0 : QmlCfg.smallMargin
    Layout.bottomMargin: senderName === "" ? QmlCfg.smallMargin : QmlCfg.margin
    Layout.preferredHeight: senderName !== "" ? QmlCfg.margin : 0
    font.bold: true
    color: outbound ? QmlCfg.palette.mainTextColor : QmlCfg.palette.iconFill
}
