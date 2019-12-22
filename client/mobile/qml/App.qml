import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import "./Headers" as Headers
import "./HomeScreen" as HomeScreen
import "./NewContactView" as NewContactView
import "./ChatView" as ChatView
import "./SettingsMenu" as SettingsMenu

Page {
    id: appRoot
    anchors.fill: parent

    readonly property alias globalTimer: globalTimer
    Timer {
        id: globalTimer
        signal refreshTime

        interval: 10000
        running: true
        repeat: true
        onTriggered: refreshTime()
    }

    header: Headers.HeadersMain {
        id: rootHeader
    }

    //TODO: Rename me
    Component {
        id: cvMain
        HomeScreen.HomeScreenMain {
            readonly property string stateName: "home"
        }
    }

    Component {
         SettingsMenu.SettingsMenuMain {
            readonly property string stateName: "config"
         }
    }

    Component {
        id: newContactViewMain
        NewContactView.NewContactViewMain {
            readonly property string stateName: "newContact"
        }
    }

    Component {
        id: newGroupViewMain
        HomeScreen.NewGroupView {
            readonly property string stateName: "newGroup"
        }
    }

    StackView {
        id: mainView
        anchors.fill: parent
        initialItem: cvMain
        onCurrentItemChanged: {
            rootHeader.state = currentItem.stateName
        }
    }

    Component.onCompleted: Herald.login()
}
