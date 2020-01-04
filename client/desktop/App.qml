import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import "./SideBar"
import "."
import "ChatView/Popups" as CvPopups
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3

Item {
    id: appRoot

    focus: true
    anchors.fill: parent.fill

    TopMenuBar {
        Popups.SettingsPopup {
            id: preferencesPopup
        }
    }

    FileDialog {
        id: attachmentDownloader
        property string filePath
        selectFolder: true
        folder: StandardPaths.writableLocation(StandardPaths.DesktopLocation)
        onAccepted: Herald.utils.saveFile(filePath, fileUrl)
        selectExisting: false
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

    Loader {
        id: messageInfoLoader
        width: active ? chatView.width : 0
        height: active ? chatView.height : 0
        anchors.top: active ? parent.top : undefined
        anchors.right: active ? parent.right : undefined
        property var convoMembers
        property var messageData
        property var ownedMessages
        active: false
        sourceComponent: CvPopups.MoreInfoPopup {
            id: moreInfo
        }
    }

    Loader {
        id: groupSettingsLoader
        width: active ? chatView.width : 0
        height: active ? chatView.height : 0
        anchors.top: active ? parent.top : undefined
        anchors.right: active ? parent.right : undefined
        property var convoData
        property var convoMembers
        property bool group
        active: false
        sourceComponent: group ? groupSettings : convoSettings

        Component {
            id: groupSettings
            CvPopups.GroupSettingsPopup {}
        }

        Component {

            id: convoSettings
            CvPopups.ConvoSettingsPopup {}
        }
    }

    Popups.ColorPicker {
        id: avatarColorPicker
    }

    Popups.SettingsPopup {
        id: settingsPopup
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
                    anchors.leftMargin: CmnCfg.megaMargin
                    anchors.verticalCenter: parent.verticalCenter
                    text: qsTr("Herald")
                    font: CmnCfg.headerBarFont
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
            property var currentConvoId
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
