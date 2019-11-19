import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import Qt.labs.settings 1.0
import "qrc:/imports" as Imports
import "SideBar/popups" as Popups
import "errors" as ErrorUtils
import QtQml 2.13

ApplicationWindow {
    id: root
    title: "Herald"

    visible: true
    width: 900
    height: 640
    minimumWidth: 500
    minimumHeight: 300

    Errors {
        id: errorQueue
        onTryPollChanged: {
            var errMsg = errorQueue.nextError()
            if (errMsg !== "") {
                errPopup.errorMsg = errMsg
                errPopup.open()
            }
        }
        property var errPopup: ErrorUtils.ErrorDialog {
        }
    }

    HeraldUtils {
        id: heraldUtils
    }

    HeraldState {
        id: heraldState
    }

    Loader {
        id: appLoader
        active: heraldState.configInit
        anchors.fill: parent
        sourceComponent: App {
        }
    }

    Loader {
        anchors.fill: parent
        id: registrationLoader
        active: !heraldState.configInit
        sourceComponent: RegistrationPage {
        }
    }
}
