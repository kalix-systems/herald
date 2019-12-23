import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common" as Common

Page {

    ColumnLayout {
        anchors.fill: parent
        spacing: CmnCfg.defaultMargin
        anchors.leftMargin: parent.width * 0.1

        Label {
            text: qsTr("Request a New Contact")
        }

        TextArea {
            id: usernameTextArea
            Layout.preferredWidth: parent.width * 0.8
            background: Rectangle {
                border.color: CmnCfg.palette.borderColor
            }
            placeholderText: qsTr("Enter a UID")
        }

        TextArea {
            Layout.alignment: Qt.AlignLeft
            Layout.preferredWidth: parent.width * 0.8
            Layout.preferredHeight: parent.height * 0.5
            placeholderText: qsTr("Enter message text")
            background: Rectangle {
                border.color: CmnCfg.palette.borderColor
            }
        }

        Button {
            text: qsTr("Send")
            onClicked: {
                Herald.users.add(usernameTextArea.text.trim())
                mainView.pop()
            }
        }

        Item {
            Layout.fillHeight: true
        }
    }
}
