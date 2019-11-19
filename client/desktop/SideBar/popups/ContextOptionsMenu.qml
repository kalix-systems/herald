import Qt.labs.platform 1.1

Menu {
    id: utilityOptionsMenu
    MenuItem {
        text: "Add contact"
        onTriggered: sideBar.sideBarState.state = "newContactState"
    }

    MenuItem {
        text: "Config settings"
        onTriggered: configPopup.show()
    }
}
