import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import "./SideBar"
import "."

Item {
    id: appRoot

    focus: true
    anchors.fill: parent.fill

    TopMenuBar {
        Popups.ConfigPopup {
            id: preferencesPopup
        }
    }

    readonly property alias globalTimer: globalTimer
    Timer {
        id: globalTimer
        signal refreshTime

        interval: 10000
        running: true
        repeat: true
        onTriggered: refreshTime()
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

    Popups.NewMessagePopup {
        id: convoMenu
    }

    Popups.ImageCropPopup {
        id: imageCrop
    }
    // TODO: move into seperate file
    Component {
        id: splash

        Rectangle {
            anchors.fill: parent
            color: CmnCfg.palette.medGrey

            Rectangle {
                anchors.top: parent.top
                width: parent.width
                color: CmnCfg.palette.offBlack
                height: CmnCfg.toolbarHeight + 1

                Rectangle {
                    anchors.left: parent.left
                    height: parent.height
                    width: 1
                    color: CmnCfg.palette.lightGrey
                }

                Text {
                    anchors.left: parent.left
                    anchors.leftMargin: CmnCfg.largeMargin
                    anchors.verticalCenter: parent.verticalCenter
                    text: qsTr("Herald")

                    font.pixelSize: CmnCfg.headerSize
                    font.family: CmnCfg.labelFont.name
                    font.bold: true
                    color: CmnCfg.palette.white
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

        // TODO: combine these two rectangles and figure out width
        handle: Item {
            id: handle
            implicitWidth: 1
            Rectangle {
                id: toolBarHandle
                implicitWidth: 1
                color: CmnCfg.palette.offBlack
                height: CmnCfg.toolbarHeight + 1
                anchors {
                    top: parent.top
                }
            }

            Rectangle {
                implicitWidth: 1
                color: CmnCfg.palette.offBlack
                anchors {
                    top: toolBarHandle.bottom
                    bottom: parent.bottom
                }
            }
        }
    }

    Component.onCompleted: Herald.login()
}
