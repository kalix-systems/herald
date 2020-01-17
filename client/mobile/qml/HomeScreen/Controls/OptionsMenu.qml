import QtQuick 2.13
import LibHerald 1.0
import QtQml 2.13
import Qt.labs.platform 1.1

Menu {
    id: optionsMenu

    MenuItem {
        text: qsTr("Archived")
        onTriggered: {
            cvMainView.state = "archiveState"
        }
    }

    MenuItem {
        text: qsTr("Settings")
        onTriggered: mainView.push(settingsMain)
    }
}
