import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common" as Common
import "Controls"

Page {

    header: NewContactHeader {}

    ColumnLayout {
        anchors.fill: parent
        spacing: CmnCfg.margin
        anchors.leftMargin: parent.width * 0.1

        Label {
            text: "Request a New Contact"
        }

        TextArea {
            id: usernameTextArea
            Layout.preferredWidth: parent.width * 0.8
            background: Rectangle {
                border.color: CmnCfg.palette.borderColor
            }
            placeholderText: "Enter a UID"
        }

        TextArea {
            Layout.alignment: Qt.AlignLeft
            Layout.preferredWidth: parent.width * 0.8
            Layout.preferredHeight: parent.height * 0.5
            placeholderText: "Enter message text"
            background: Rectangle {
                border.color: CmnCfg.palette.borderColor
            }
        }

        Button {
            text: "Send"
            onClicked: {
                herald.users.add(usernameTextArea.text.trim())
                mainView.pop()
            }
        }

        Item {
            Layout.fillHeight: true
        }
    }
}
