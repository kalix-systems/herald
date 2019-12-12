import Qt.labs.platform 1.1

Menu {
    id: utilityOptionsMenu
    MenuItem {
        text: qsTr("Add contact")
        onTriggered: sideBar.sideBarState.state = "newContactState"
    }

    MenuItem {
        text: qsTr("Config settings")
        onTriggered: configPopup.show()
    }
}
