import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0
import "./Errors" as Errors
import "./LoginPage" as LoginPage

ApplicationWindow {
    id: root
    visible: true
    width: 300
    height: 500

    // utility code, meant to reduce the amount of js laying
    // around the code base
    HeraldState {
        id: heraldState
    }

    Loader {
        id: capitan
        active: false
        sourceComponent: Item {
        }
    }

    // displays error dialog upon output from
    // libherald, meant as a debugging tool
    Errors.ErrorHandler {
    }

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

    Loader {
        id: appLoader
        active: heraldState.configInit
        anchors.fill: parent
        sourceComponent: App {
        }
    }
}
