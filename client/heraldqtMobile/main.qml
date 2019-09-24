import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0

ApplicationWindow {
    id: root
    visible: true
    width: 350
    height: 550

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
        sourceComponent: App {}
    }

    Loader {
        anchors.fill: parent
        id: loginLoader
        active: !heraldState.configInit
        sourceComponent: LoginPage {
        }
    }

}
