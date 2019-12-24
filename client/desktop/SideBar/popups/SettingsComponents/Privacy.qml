import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../../../common" as CMN
import "qrc:/imports"
import Qt.labs.platform 1.0

ColumnLayout {

    RowLayout {
        Layout.fillWidth: true
        StandardLabel {
            text: qsTr("Default message expiration time: ") + expirationMenu.currentSelection
            color: "black"
            Layout.leftMargin: CmnCfg.defaultMargin
            font.pixelSize: CmnCfg.chatTextSize
        }

        ButtonForm {
            source: "qrc:/dropdown-arrow-icon.svg"
            onClicked: expirationMenu.open()
        }

        Item {
            Layout.fillWidth: true
        }

        //TODO: THIS SHOULD COME FROM THE CONFIG MODEL
        Menu {
            id: expirationMenu
            property string currentSelection: qsTr("None Selected")
            MenuItem {
                text: qsTr("Off")
                checkable: true
                checked: text === expirationMenu.currentSelection
                onTriggered: expirationMenu.currentSelection = text
            }
            MenuItem {
                text: qsTr("1 minute")
                checkable: true
                checked: text === expirationMenu.currentSelection
                onTriggered: expirationMenu.currentSelection = text
            }
            MenuItem {
                text: qsTr("1 hour")
                checkable: true
                checked: text === expirationMenu.currentSelection
                onTriggered: expirationMenu.currentSelection = text
            }

            MenuItem {
                text: qsTr("1 day")
                checkable: true
                checked: text === expirationMenu.currentSelection
                onTriggered: expirationMenu.currentSelection = text
            }

            MenuItem {
                text: qsTr("1 week")
                checkable: true
                checked: text === expirationMenu.currentSelection
                onTriggered: expirationMenu.currentSelection = text
            }

            MenuItem {
                text: qsTr("1 month")
                checkable: true
                checked: text === expirationMenu.currentSelection
                onTriggered: expirationMenu.currentSelection = text
            }

            MenuItem {
                text: qsTr("1 year")
                checkable: true
                checked: text === expirationMenu.currentSelection
                onTriggered: expirationMenu.currentSelection = text
            }
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
