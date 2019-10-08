import QtQuick 2.13
import QtQuick.Layouts 1.12

Rectangle {
    id: reply
    property string text: ""
    property string op: ""
    property var messageId: 0
    color: "green"
    radius: 5 // jh this is magic
    height: content.height
    width: content.width
    ColumnLayout {
        id: content
        TextEdit {
            Layout.fillWidth: true
            Layout.minimumWidth: 200
            Layout.margins: cfgSmallMargins
            text: "greeee"
            readOnly: true
            wrapMode: TextEdit.Wrap
        }
    }
}
