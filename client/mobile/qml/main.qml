import QtQuick 2.13
import QtQuick.Controls 2.12
import Qt.labs.platform 1.1
import LibHerald 1.0
import "qrc:/imports/errors"
import "./LoginPage" as LoginPage

ApplicationWindow {
    id: root
    visible: true
    width: 300
    height: 500

    ErrorDialog {
        id: errPopup

        Connections {
            target: Herald.errors
            onTryPollChanged: {
                const errMsg = Herald.errors.nextError()

                if (errMsg !== "") {
                    errPopup.errorMsg = errMsg
                    errPopup.open()
                }
            }
        }
    }

    MobileHelper {
        Component.onCompleted: {
            set_status_bar_color(CmnCfg.palette.offBlack)
        }
    }

    Loader {
        id: capitan
        active: false
        sourceComponent: Item {}
    }

    Loader {
        id: loginPageLoader
        active: !Herald.configInit
        anchors.fill: parent
        // windows cannot be filled, unless reffered to as parent
        sourceComponent: LoginPage.LoginLandingPage {
            id: lpMain
            anchors.fill: parent
        }
    }

    Loader {
        id: appLoader
        active: Herald.configInit
        anchors.fill: parent
        sourceComponent: App {}
    }
}
