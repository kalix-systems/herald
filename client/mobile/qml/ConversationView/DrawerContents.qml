import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import "../Common"

Item {
    anchors.fill: parent
    Item {
        anchors.centerIn: parent

        Row {
            Label {
                text: "Add Contact :"
                anchors.verticalCenter: addContactButton.verticalCenter
            }
            IconButton {
                id: addContactButton
                imageSource: "qrc:/add-contact-icon.svg"
                tapCallback: function () {
                    mainView.push(newContactViewMain)
                    contextDrawer.close()
                }
            }
        }
    }
}
