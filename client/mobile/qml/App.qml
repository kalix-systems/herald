import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import "./HomeScreen" as HomeScreen
import "./NewContactView" as NewContactView
import "./ChatView" as ChatView
import "./ConfigMenu" as ConfigMenu

Item {
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

    Component {
        id: cvMain
        HomeScreen.HomeScreenMain {}
    }

    Component {
        id: configMain
        ConfigMenu.ConfigMenuMain {}
    }

    Component {
        id: newContactViewMain
        NewContactView.NewContactViewMain {}
    }

    Component {
        id: newGroupViewMain
        HomeScreen.NewGroupView {}
    }

    StackView {
        id: mainView
        anchors.fill: parent
        initialItem: cvMain
    }

    Component.onCompleted: Herald.login()
}
