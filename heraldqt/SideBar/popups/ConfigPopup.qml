import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.13
import QtQuick.Window 2.2
import LibHerald 1.0
import "../../common" as Common
import "../../common/utils.mjs" as Utils
import "./ConfigPopupSubmission.mjs" as JS

Window {
    id: configPopup

    width: 600
    height: 200
    maximumHeight: height
    minimumHeight: height
    maximumWidth: width
    minimumWidth: width

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
            config.profile_picture = fileUrl
            print("set to", fileUrl)
        }
    }

    TabBar {
        width: parent.width
        height: 50
        id: bar
        TabButton {
            text: qsTr("Account")
        }
        TabButton {
            text: qsTr("UI")
        }
        TabButton {
            text: qsTr("Authentication")
        }
        TabButton {
            text: qsTr("Notifications")
        }
    }

    StackLayout {
        width: parent.width
        currentIndex: bar.currentIndex
        anchors.top: bar.bottom
        ColumnLayout {
            id: accountPreferences
            Layout.alignment: Qt.AlignCenter
            Layout.fillWidth: true
            /// RS: check with the server to prevent duplicate ID's
            RowLayout {
                TextField {
                    id: cfgUid
                    enabled: !config.configId
                    property bool userIdValid: true
                    placeholderText: enabled ? "Enter UID " : config.configId
                    selectionColor: QmlCfg.palette.tertiaryColor
                }

                TextField {
                    id: cfgUname
                    maximumLength: 256
                    property bool usernameValid: true
                    placeholderText: "Enter Username"
                    selectionColor: QmlCfg.palette.tertiaryColor
                }
            }

            Button {
                text: "select profile picture"
                onClicked: {
                    cfgPfp.open()
                }
            }

            Button {
                text: "Submit"
                onClicked: {
                    JS.submit(config, cfgUid, cfgUname)
                    close()
                }
            }
        }
        Item {
            id: uiPreferences
        }
        Item {
            id: authenticationPreferences
        }
        Item {
            id: notificationPreferences
        }


    }
}
