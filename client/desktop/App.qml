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
        Popups.ContactsPopup {
            id: contactsPopup
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

        interval: 1000
        running: true
        repeat: true
        onTriggered: refreshTime()
    }

    Loader {
        id: messageInfoLoader
        width: active ? chatView.width * 0.75 : 0
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
        width: active ? chatView.width * 0.75 : 0
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

    //    Popups.ImageCropPopup {//  id: imageCrop
    //    }
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
                    font: CmnCfg.headerFont
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

    Item {
        id: rootSplitView
        anchors.fill: parent
        property var splitHandle: splitHandleDrag

        function splitHandleDrag(mouseX) {
            if ((sideBar.width + mouseX) >= root.width * 0.6)
                return
            if ((sideBar.width + mouseX) <= root.width * 0.25)
                return

            sideBar.width += mouseX
        }

        // filler mouse area to pin split view cursor when in resize mode
        MouseArea {
            anchors.fill: parent
            enabled: mouse.drag.active
            cursorShape: if (enabled)
                             Qt.SplitHCursor
            z: enabled ? sideBar.z + 1 : rootSplitView.z - 1
        }

        SideBarMain {
            id: sideBar
            anchors.left: parent.left
            height: parent.height
            width: 300
        }

        Loader {
            anchors.left: sideBar.right
            id: chatView
            property var currentConvoId
            sourceComponent: splash
            width: appRoot.width - sideBar.width
            height: appRoot.height

            Rectangle {

                anchors.horizontalCenter: parent.left
                //  anchors.rightMargin: 2
                width: 9
                height: parent.height
                color: "transparent"

                Rectangle {
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                    width: 5
                    height: parent.height - (CmnCfg.toolbarHeight + 2)
                    color: CmnCfg.palette.offBlack
                    z: parent.z + 1
                }
                MouseArea {
                    id: mouse
                    drag.target: parent
                    anchors.fill: parent
                    drag.axis: Drag.XAxis
                    cursorShape: Qt.SplitHCursor
                    preventStealing: drag.active
                    drag.threshold: 0

                    onMouseXChanged: {
                        if (drag.active)
                            rootSplitView.splitHandle(mouseX)
                    }
                }
            }
        }
    }

    Component.onCompleted: Herald.login()
}
