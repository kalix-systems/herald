import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Dialogs 1.3

MessageDialog {
    property string errorMsg
    title: qsTr("Error")
    text: errorMsg
}
