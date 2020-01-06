import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import "./Headers" as Headers
import "./HomeScreen" as HomeScreen
import "./NewContactView" as NewContactView
import "./ChatView" as ChatView
import "./GlobalSearch" as GlobalSearch
import "qrc:/imports/Settings" as Settings

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
        HomeScreen.HomeScreenMain {}
    }

    Component {
        id: messageInfoMain
        ChatView.InfoPage {}
    }

    Component {
        id: settingsMain
        Settings.SettingsPane {
            readonly property Component headerComponent: Headers.SettingsHeader {}
        }
    }

    Component {
        id: newContactViewMain
        NewContactView.NewContactViewMain {}
    }

    Component {
        id: newGroupViewMain
        HomeScreen.NewGroupView {}
    }

    Component {
        id: globalSearchView
        GlobalSearch.GlobalSearchMain {}
    }

    StackView {
        id: mainView
        anchors.fill: parent
        initialItem: cvMain
        onCurrentItemChanged: {
            // upon pushing a page set the header to the proper component
            rootHeader.headerComponent = currentItem.headerComponent
        }
    }

    Component.onCompleted: Herald.login()
}
