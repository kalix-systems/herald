import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0
import "./ConversationView" as CVView
import "./ChatView" as ChatView
import "./Errors" as Errors
import "./LoginPage" as LoginPage

ApplicationWindow {
    id: root

    visible: true
    width: 300
    height: 500

    Component {
        id: cvMain
        CVView.ConversationViewMain {}
    }
    // utility code, meant to reduce the amount of js laying
    // around the code base
    HeraldUtils {
        id: heraldUtils
    }

    Conversations {
        id: conversationsModel
    }

    HeraldState {
        id: heraldState
    }

    Config {
        id: configModel
    }

    Users {
        id: usersModel
    }

    // displays error dialog upon output from
    // libherald, meant as a debugging tool
    Errors.ErrorHandler {}

    Loader {
        id: loginPageLoader
        active: !heraldState.configInit
        anchors.fill: parent
        // windows cannot be filled, unless reffered to as parent
        sourceComponent: LoginPage.LoginLandingPage {
            id: lpMain
            anchors.fill: parent
        }
    }

    StackView {
        id: mainView
        anchors.fill: parent
        initialItem: cvMain
    }
}
