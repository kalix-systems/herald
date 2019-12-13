import LibHerald 1.0
import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Label {
    text: knownReply ? Herald.users.nameById(modelData.opAuthor) : ""
    font.bold: true
    color: opColor

    Layout.preferredHeight: knownReply ? implicitHeight : 0

    background: Rectangle {
        opacity: 1
        color: "red"
    }
}
