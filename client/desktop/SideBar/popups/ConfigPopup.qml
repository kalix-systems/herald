import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.13
import QtQuick.Window 2.2
import LibHerald 1.0
import "../../common" as Common
import "./js/ConfigPopupSubmission.mjs" as JS

Window {
    id: configPopup
    width: CmnCfg.configWidth
    height: CmnCfg.configHeight

    Component.onCompleted: {
        x = root.x + root.width / 3
        y = root.y + 100
    }

    FileDialog {
        id: cfgPfp
        property bool pfpValid: true
        folder: shortcuts.desktop
        nameFilters: ["(*.jpg *.png *.jpeg)"]
        onSelectionAccepted: {
            herald.config.profilePicture = fileUrl
            print("set to", fileUrl)
        }
    }
    SplitView {
        anchors.fill: parent

        handle: Item {
            id: handle
            implicitWidth: 1
            Rectangle {
                id: toolBarHandle
                implicitWidth: 1
                color: CmnCfg.palette.medGrey
                height: CmnCfg.toolbarHeight
                anchors {
                    top: parent.top
                }
            }
            Rectangle {
                implicitWidth: 1
                color: CmnCfg.palette.black
                anchors {
                    top: toolBarHandle.bottom
                    bottom: parent.bottom
                }
            }
        }

        Page {
            SplitView.preferredWidth: 0.3 * 600
            SplitView.minimumWidth: label.width
            SplitView.preferredHeight: parent.height
            header: Rectangle {
                color: CmnCfg.palette.offBlack
                height: CmnCfg.toolbarHeight
                Row {
                    leftPadding: CmnCfg.margin
                    anchors.fill: parent
                    Label {
                        id: label
                        text: qsTr("Settings")
                        color: CmnCfg.palette.white
                        font.pixelSize: CmnCfg.headerSize
                        font.family: CmnCfg.labelFont.name
                        font.bold: true
                        anchors.verticalCenter: parent.verticalCenter
                        elide: Label.ElideRight
                    }
                }
            }

            RowLayout {
                anchors.fill: parent

                Rectangle {
                    Layout.fillHeight: true
                    width: 1
                    color: CmnCfg.palette.offBlack
                }

                ScrollView {
                    Layout.fillWidth: true
                }
            }
        }

        Page {

            SplitView.preferredWidth: 600 / 4
            SplitView.minimumWidth: label.width
            SplitView.preferredHeight: parent.height
            header: Rectangle {
                color: CmnCfg.palette.offBlack
                height: CmnCfg.toolbarHeight
                Row {
                    leftPadding: CmnCfg.margin
                    anchors.fill: parent
                }
            }
        }
    }

    //    TabBar {
    //        width: parent.width
    //        height: 50
    //        id: bar
    //        TabButton {
    //            text: qsTr("Account")
    //        }
    //        TabButton {
    //            text: qsTr("UI")
    //        }
    //        TabButton {
    //            text: qsTr("Authentication")
    //        }
    //        TabButton {
    //            text: qsTr("Notifications")
    //        }
    //    }

    //    StackLayout {
    //        width: parent.width
    //        currentIndex: bar.currentIndex
    //        anchors.top: bar.bottom
    //        ColumnLayout {
    //            id: accountPreferences
    //            Layout.alignment: Qt.AlignCenter
    //            Layout.fillWidth: true
    //            /// RS: check with the server to prevent duplicate ID's
    //            RowLayout {
    //                TextField {
    //                    id: cfgUid
    //                    enabled: false
    //                    property bool userIdValid: true
    //                    placeholderText: enabled ? "Enter UID " : herald.config.configId
    //                    selectionColor: "lightsteelblue"
    //                }

    //                TextField {
    //                    id: cfgUname
    //                    maximumLength: 256
    //                    property bool usernameValid: true
    //                    text: herald.config.name
    //                    selectionColor: "lightsteelblue"
    //                }
    //            }

    //            Button {
    //                text: "select profile picture"
    //                onClicked: cfgPfp.open()
    //            }

    //            Button {
    //                text: "Submit"
    //                onClicked: {
    //                    JS.submit(herald.config, cfgUname)
    //                    close()
    //                }
    //            }
    //        }
    //        Item {
    //            id: uiPreferences
    //            Button {
    //                text: "toggle solarized dark"
    //                onClicked: CmnCfg.theme = CmnCfg.theme === 1 ? 0 : 1
    //            }
    //        }
    //        Item {
    //            id: authenticationPreferences
    //        }
    //        Item {
    //            id: notificationPreferences
    //        }
    //    }
}
