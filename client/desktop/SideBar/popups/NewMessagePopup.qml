import Qt.labs.platform 1.1

Menu {
    id: convoMenu
    MenuItem {
        text: "New group conversation"
        onTriggered: {
            sideBar.sideBarState.state = "newGroupState"
        }
    }
}
