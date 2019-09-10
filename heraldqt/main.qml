import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import Qt.labs.platform 1.0
import QtQml 2.13

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB

ApplicationWindow {
    id: root
    visible: true
    width: 900
    height: 640
    title: qsTr("Herald")
    minimumWidth: 250
    minimumHeight: 300

    // OSD
    MenuBar {
        Menu {
            title: "Herald"
            MenuItem {
                text: "Preferences"
                onTriggered: preferencesPopup.open()
            }
        }
        Menu {
            title: "Window"
            MenuItem {
                text: "Minimize"
                onTriggered: root.showMinimized()
            }
        }
    }

    NetworkHandle {
        id: networkHandle
    }

    Popups.ConfigPopup {
        id: preferencesPopup
    }

    Messages {
        id: messageModel
    }

    Contacts {
         id: contactsModel
        }

    // NPB : always instantiated, more like a state, or a page than a popup
    Config {
        id: config
        Component.onCompleted: {
            if (!config.exists()) {
                preferencesPopup.open()
                print("placeholder for a popup which forces first time config.")
            }
        }
    }

    SplitView {
        id: rootSplitView
        anchors.fill: parent
        Layout.fillWidth: true
        Layout.fillHeight: true
        orientation: Qt.Horizontal

        SideBar {
            id: sideBar
        }

        ChatView {
            id: chatView
        }

        handle: Rectangle {
            implicitWidth: 2
            implicitHeight: 4
            color: QmlCfg.palette.secondaryColor
        }

    }

}
