import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12

//bundles together textedit and text because textmetrics is broken
TextEdit {
    //a replacement for textmetrics width
    property alias correctWidth: dummyText.width
    id: textEdit
    Text {
        text: textEdit.text
        id: dummyText
        visible: false
    }
}
