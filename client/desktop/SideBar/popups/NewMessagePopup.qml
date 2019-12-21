import Qt.labs.platform 1.1

Menu {
    id: convoMenu
    MenuItem {
        text: qsTr("New group conversation")
        onTriggered: sideBar.sideBarState.state = "newGroupState"
    }
}
