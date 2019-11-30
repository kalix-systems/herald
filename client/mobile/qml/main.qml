import QtQuick 2.13
import QtQuick.Controls 2.12
import Qt.labs.platform 1.1
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
    Herald {
        id: herald

        // NOTE: This is very important. Until our initialization is cleaned up this has to happen immediately after `Herald`
        // is initialized.
        Component.onCompleted: herald.setAppLocalDataDir(StandardPaths.writableLocation(StandardPaths.AppLocalDataLocation))
    }

    Loader {
        id: capitan
        active: false
        sourceComponent: Item {
        }
    }

    Loader {
        id: loginPageLoader
        active: !herald.configInit
        anchors.fill: parent
        // windows cannot be filled, unless reffered to as parent
        sourceComponent: LoginPage.LoginLandingPage {
            id: lpMain
            anchors.fill: parent
        }
    }

    Loader {
        id: appLoader
        active: herald.configInit
        anchors.fill: parent
        sourceComponent: App {
        }
    }
}
