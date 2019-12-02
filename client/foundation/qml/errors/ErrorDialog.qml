import QtQuick 2.0
import QtQuick.Dialogs 1.2
import LibHerald 1.0

MessageDialog {
    property string errorMsg

    title: qsTr("Error")
    text: errorMsg
    icon: StandardIcon.Warning
}
