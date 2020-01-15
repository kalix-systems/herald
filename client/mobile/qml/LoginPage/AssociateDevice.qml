import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.12

Page {

    ColumnLayout {
        anchors.fill: parent

        Label {
            id: instructions
        }

        Label {
            id: genText
        }

        RowLayout {
            Button {
                id: cancelButton
                text: qsTr("Cancel")
                onClicked: loginPageLoader.sourceComponent = lpMain
            }
            Button {
                id: submitButton
                text: qsTr("Submit")
            }
        }
    }
}
