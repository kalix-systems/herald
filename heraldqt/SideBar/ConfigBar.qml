import QtQuick 2.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0

ToolBar {
    id: toolBar
    anchors {
        left: parent.left
        top: parent.top
    }
    width: contactPane.width
    height: 40

    background: Rectangle {
        color: QmlCfg.palette.mainColor
        border.color: QmlCfg.palette.secondaryColor
    }

    /// unpolished temporary Popup
    Popup {
        id: configPopup
        modal: true
        focus: true
        width: 300
        height: 300
        x: (root.width - width) / 2
        y: (root.height - height) / 2
        Column {
            ///  TODO : This field should really not exist but it had to be here
            /// until a hero fixes the server
            ///  TODO : Also write a validator object which sanitizes all input
            TextField {
                id: cfgUid
                property bool userIdValid: true
                placeholderText: "Enter UID "
            }

            TextField {
                id: cfgUname
                property bool usernameValid: true
                placeholderText: "Enter Username"
            }

            FileDialog {
                id: cfgPfp
                property bool pfpValid: true
            }

            Button {
                text: "select profile picture"
                onClicked: {
                    cfgPfp.open()
                }
            }
        }
        Button {
            anchors.bottom: parent.bottom
            text: "Submit"
            enabled: cfgUid.userIdValid && cfgUname.usernameValid
                     && cfgPfp.pfpValid
            onClicked: {

                configPopup.close()
            }
        }
    }

    Button {
        height: parent.height
        width: height
        anchors.right: parent.right
        background: Image {
            source: "qrc:///icons/gear.png"
            width: parent.height
            height: width
            scale: 0.7
        }
        onClicked: {
            configPopup.open()
        }
    }
}
