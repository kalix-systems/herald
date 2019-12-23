import Qt.labs.platform 1.1

Menu {
    id: utilityOptionsMenu

    MenuItem {
        text: qsTr("Settings")
        onTriggered: settingsPopup.show()
    }
}
