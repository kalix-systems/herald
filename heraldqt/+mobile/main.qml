import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import QtQml 2.13

ApplicationWindow {
    id: root
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
        Layout.fillWidth: true
        Layout.fillHeight: true
        sourceComponent: App {
        }
    }

    Loader {
        anchors.fill: parent
        id: loginLoader
        active: !heraldState.configInit
        sourceComponent: LoginPage {
        }
    }
}
