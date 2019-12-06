import QtQuick 2.0
import LibHerald 1.0
import Qt.labs.platform 1.1

MessageDialog {
    property string errorMsg
    title: qsTr("Error")
    text: errorMsg
}
