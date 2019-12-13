import QtQuick 2.13
import Qt.labs.platform 1.1

// note: this really only works on macOS
MenuBar {
    Menu {
        MenuItem {
            text: qsTr("Preferences") + "..."
            shortcut: StandardKey.Preferences
            onTriggered: preferencesPopup.show()
        }
    }

    // TODO: when we have memory for chatTextAreas bind these events to them
    Menu {
        title: qsTr("Edit")
        MenuItem {
            text: qsTr("Undo")
            shortcut: StandardKey.Undo
            onTriggered: print("does Nothing")
        }
        MenuItem {
            text: qsTr("Redo")
            shortcut: StandardKey.Redo
            onTriggered: print("does Nothing")
        }
    }
    Menu {
        title: qsTr("View")
        MenuItem {
            text: qsTr("Minimize")
            onTriggered: root.showMinimized()
        }
        MenuItem {
            text: qsTr("Fullscreen")
            shortcut: StandardKey.FullScreen
            onTriggered: root.showFullScreen()
        }
    }
}
