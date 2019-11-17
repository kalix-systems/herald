import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import "./SideBar"

Item {
    id: appRoot

    anchors.fill: parent.fill
    TopMenuBar {
        Popups.ConfigPopup {
            id: preferencesPopup
        }
    }

    Users {
        id: contactsModel
    }

    Conversations {
        id: conversationsModel
    }

    Popups.ColorPicker {
        id: avatarColorPicker
    }

    Popups.ConfigPopup {
        id: configPopup
    }

    Popups.ContextOptionsMenu {
        id: contextOptionsMenu
    }

    Config {
        id: config
    }

    focus: true


    Component {
        id: splash

        Rectangle {
            anchors.fill: parent
            color: CmnCfg.palette.sideBarHighlightColor

            Rectangle {
                anchors.top: parent.top
                width: parent.width
                color: CmnCfg.palette.secondaryColor
                height: CmnCfg.toolbarHeight + 1

                Text {
                    anchors.left: parent.left
                    anchors.leftMargin: CmnCfg.largeMargin
                    anchors.verticalCenter: parent.verticalCenter
                    text: "Herald"
                    font.pixelSize: CmnCfg.headerSize
                    font.family: CmnCfg.chatFont.name
                    font.bold: true
                    color: CmnCfg.palette.mainColor
                }
            }

            Image {
                anchors.centerIn: parent
                source: "qrc:/herald.icns"
                mipmap: true

            }
        }
    }



    SplitView {
        id: rootSplitView
        anchors.fill: parent
        orientation: Qt.Horizontal

        SideBarMain {
            id: sideBar
        }

        Loader {
            id: chatView
            sourceComponent: splash
        }

        handle: Item {
            id: handle
            implicitWidth: 1
            Rectangle {
                id: toolBarHandle
                implicitWidth: 1
                color: "white"
                height: CmnCfg.toolbarHeight
                anchors {
                    top: parent.top
                }
            }
            Rectangle {
                implicitWidth: 1
                color: CmnCfg.palette.borderColor
                anchors {
                    top: toolBarHandle.bottom
                    bottom: parent.bottom
                }
            }
        }
    }

    Component.onCompleted: heraldState.login()
}
