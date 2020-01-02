import QtQuick 2.14
import QtQuick.Controls 2.14
import Qt.labs.platform 1.1
import "qrc:/imports/errors"
import "./LoginPage" as LoginPage
import LibHerald 1.0

ApplicationWindow {
    id: root
    visible: true
    // for desktop prototyping
    // removed on desktop
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
        id: mobHelper
        Component.onCompleted: {
            set_status_bar_color(CmnCfg.palette.offBlack)
            open_gallery()
        }
    }

    Loader {
        id: loginPageLoader
        active: !Herald.configInit
        anchors.fill: parent
        // windows cannot be filled, unless referred to as parent
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
