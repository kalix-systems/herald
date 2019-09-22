import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import QtQml 2.13

ApplicationWindow {
    id: root
    width: 350
    height: 550
    visible: true

    // This provides a few purely functional helper methods
    HeraldUtils {
        id: heraldUtils
    }

    HeraldState {
        id: heraldState
        onConfigInitChanged: {
            appLoader.active = !appLoader.active
            loginLoader.active = !loginLoader.active
        }
    }

    Loader {
        id: appLoader
        active: heraldState.configInit
        anchors.fill: parent
        sourceComponent: App {
        }
    }

    Loader {
        id: loginLoader
        anchors.fill: parent
        active: !heraldState.configInit
        sourceComponent: LoginPage {
        }
    }
}
