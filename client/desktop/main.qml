import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import Qt.labs.settings 1.0
import Qt.labs.platform 1.1
import "qrc:/imports" as Imports
import "qrc:/imports/errors"
import "SideBar/popups" as Popups
import QtQml 2.13

ApplicationWindow {
    id: root
    title: "Herald"
    visible: true
    width: 900
    height: 640
    minimumWidth: 500
    minimumHeight: 300

    Herald {
        id: herald

        property var errPopup: ErrorDialog {
        }

        errors.onTryPollChanged: {
            var errMsg = herald.errors.nextError()
            if (errMsg !== "") {
                errPopup.errorMsg = errMsg
                errPopup.open()
            }
        }

        // NOTE: This is very important.
        // Until our initialization is cleaned up this has to happen immediately after `Herald`
        // is initialized.
        //
        // TODO: This should probably be called from C++ before executing the application with a manually exposed
        // extern "C" function.
        Component.onCompleted: herald.setAppLocalDataDir(
                                   StandardPaths.writableLocation(
                                       StandardPaths.AppLocalDataLocation))
    }

    Loader {
        id: appLoader
        active: herald.configInit
        anchors.fill: parent
        sourceComponent: App {
        }
    }

    Loader {
        anchors.fill: parent
        id: registrationLoader
        active: !herald.configInit
        sourceComponent: RegistrationPage {
        }
    }
}
