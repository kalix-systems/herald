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
    width: 900
    height: 640
    title: qsTr("Herald")
    minimumWidth: 500
    minimumHeight: 300

    TopMenuBar {
    }

    NetworkHandle {
        id: networkHandle
    }

    Messages {
        id: messageModel
    }

    Contacts {
        id: contactsModel
    }

    Popups.ConfigPopup {
        id: preferencesPopup
    }

    Config {
        id: config
    }

    Loader {
        anchors.fill: parent
        id: loginLoader
        active: !!!config.init
        sourceComponent: LoginPage {}
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
            color: QmlCfg.palette.secondaryColor
        }

        states: [
            State {
                when: !!!config.init
                name: "loginPage"
                PropertyChanges {
                    target: rootSplitView
                    visible : false
                }
                PropertyChanges {
                    target: root
                    height: 500
                    width: 700
                }
            },
            State {
                when: config.init
                name: "loggedIn"
                PropertyChanges {
                    target: rootSplitView
                    visible : true
                }
                PropertyChanges {
                    target: root
                    minimumWidth: 250
                    minimumHeight: 300
                    width: 900
                    height: 640
                }
            }
        ]

        transitions: Transition {
             NumberAnimation { properties: "width, height"; easing.type: Easing.InOutQuad }
         }
    }
}
