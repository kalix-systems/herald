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
