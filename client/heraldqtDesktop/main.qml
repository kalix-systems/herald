import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import Qt.labs.settings 1.0
import "SideBar/popups" as Popups
import QtQml 2.13

ApplicationWindow {
    id: root
    visible: true
    width: 900
    height: 640
    title: qsTr("Herald")
    minimumWidth: 500
    minimumHeight: 300


    TopMenuBar {
    }


    // This provides a few purely functional helper methods
    HeraldUtils {
        id: heraldUtils
    }

    HeraldState {
        id: heraldState

        onConfigInitChanged: {
            appLoader.active = !appLoader.active
            registrationLoader.active = !registrationLoader.active
        }
    }

    NetworkHandle {
        id: networkHandle
        // every conversation has it's own refresh signal. guards
        //        onNewMessageChanged: convModel.refresh()
        //onNewContactChanged: contactsModel.refresh()
        //onNewConversationChanged: conversationsModel.hardRefresh()
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
        id: registrationLoader
        active: !heraldState.configInit
        sourceComponent: RegistrationPage {
        }
    }
}
