import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "../../../common" as CMN
import "qrc:/imports"

ColumnLayout {

    RowLayout {
        Layout.fillWidth: true
        StandardLabel {
            text: qsTr("Theme")
            color: "black"
            Layout.leftMargin: CmnCfg.margin
            font.pixelSize: 14
        }

        Item {
            Layout.fillWidth: true
        }

        ConfRadio {
            Layout.alignment: Qt.AlignRight
            text: qsTr("Dark")
        }

        ConfRadio {
            Layout.alignment: Qt.AlignRight
            text: qsTr("Light")
            checked: true
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }

    RowLayout {
        Layout.leftMargin: CmnCfg.margin

        Button {
            text: qsTr("Change display name")
            onClicked: {
                submissionCol.visible = true
            }
        }

        StandardLabel {
            color: "black"
            text: qsTr("Current display name: ") + (herald.config.name)
            Layout.leftMargin: CmnCfg.margin
            font.pixelSize: 14
        }
    }

    ColumnLayout {
        id: submissionCol
        Layout.leftMargin: CmnCfg.margin
        visible: false
        TextArea {
            id: displayNameArea
            placeholderText: qsTr("Enter New Display Name...")
            background: Rectangle {
                color: CmnCfg.palette.lightGrey
            }
        }

        Button {
            text: qsTr("Submit")
            onClicked: {
                submissionCol.visible = false
                herald.config.name = displayNameArea.text
            }
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }

    FileDialog {
        id: cfgPfp
        property bool pfpValid: true
        folder: shortcuts.desktop
        nameFilters: ["(*.jpg *.png *.jpeg)"]
        onSelectionAccepted: herald.config.profilePicture = fileUrl
    }

    RowLayout {
        Layout.leftMargin: CmnCfg.margin
        Button {
            text: qsTr("Select profile picture")
            onClicked: cfgPfp.open()
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
