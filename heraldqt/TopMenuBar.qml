import QtQuick 2.13
import Qt.labs.platform 1.1

// note: this will not work on windows
MenuBar {
    Menu {
       MenuItem {
            text: "Preferences..."
            shortcut: StandardKey.Preferences
             onTriggered: preferencesPopup.open()
        }
    }
    // TODO: when we have memory for chatTextAreas bind these events to them
    Menu {
        title: "Edit"
        MenuItem {
            text: "Undo"
            shortcut: StandardKey.Undo
            onTriggered: print("does Nothing")
        }
        MenuItem {
            text: "Redo"
            shortcut: StandardKey.Redo
            onTriggered: print("does Nothing")
        }
    }
    Menu {
        title: "View"
        MenuItem {
            text: "Minimize"
            onTriggered: root.showMinimized()
        }
    }
}
