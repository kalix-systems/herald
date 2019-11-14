import QtQuick 2.12
import QtQuick.Controls 2.12
import LibHerald 1.0
import "./ConversationView" as CVView
import "./NewContactView" as NewContactView
import "./ChatView" as ChatView
import "./ConfigMenu" as ConfigMenu

Item {
    id: appRoot
    anchors.fill: parent

    Component {
        id: cvMain
        CVView.ConversationViewMain {}
    }

    Component {
        id: configMain
        ConfigMenu.ConfigMenuMain {}
    }

    Component {
        id: newContactViewMain
        NewContactView.NewContactViewMain {}
    }

    HeraldUtils {
        id: heraldUtils
    }

    Conversations {
        id: conversationsModel
    }

    Config {
        id: configModel
    }

    Users {
        id: usersModel
    }

    StackView {
        id: mainView
        anchors.fill: parent
        initialItem: cvMain
    }

    Component.onCompleted: heraldState.login()
}
