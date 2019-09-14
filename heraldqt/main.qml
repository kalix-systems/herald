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

    property var gsContactId

    TopMenuBar {
    }

    property alias networkHandle: networkHandleLoader.item

    Loader {
        id: networkHandleLoader
        active: !!config.init
        sourceComponent: NetworkHandle {
            onNewMessageChanged: {
                messageModel.pollDataBase(messageModel.conversationId)
            }
        }
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
        sourceComponent: LoginPage {
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
            color: QmlCfg.palette.secondaryColor
        }

        states: [
            State {
                when: !!!config.init
                name: "loginPage"
                PropertyChanges {
                    target: rootSplitView
                    visible: false
                }
            },
            State {
                when: config.init
                name: "loggedIn"
                PropertyChanges {
                    target: rootSplitView
                    visible: true
                }
            }
        ]

        transitions: Transition {
            NumberAnimation {
                properties: "width, height"
                easing.type: Easing.InOutQuad
            }
        }
    }
}
