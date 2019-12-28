import Qt.labs.platform 1.1

Menu {
    id: convoMenu
    MenuItem {
        text: qsTr("New group")
        onTriggered: sideBar.sideBarState.state = "newGroupState"
    }

    MenuItem {
        text: qsTr("Add contact")
        onTriggered: sideBar.sideBarState.state = "newContactState"
    }
}
