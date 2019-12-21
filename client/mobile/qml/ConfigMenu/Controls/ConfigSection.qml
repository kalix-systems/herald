import QtQuick 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../Common" as Common

ColumnLayout {
    id: contentCol
    property string title
    property Component content
    width: parent.width

    Common.Divider {
        Layout.fillWidth: true
        Layout.alignment: Qt.AlignCenter
    }

    Loader {
        Layout.fillWidth: true
        sourceComponent: content
    }
}
