import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "../../common" as Common
import "../../common/utils.js" as Utils

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
            enabled: !config.config_id
            property bool userIdValid: true
            placeholderText: enabled ? "Enter UID " : config.config_id
        }

        TextField {
            id: cfgUname
            property bool usernameValid: true
            placeholderText: "Enter Username"
        }

        FileDialog {
            id: cfgPfp
            folder: shortcuts.home
            property bool pfpValid: true
            onSelectionAccepted: {
                config.profile_picture = profile_picture
                print("No PFP api right now")
            }
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
        onClicked: {
            if (!config.config_id) {
                config.config_id = cfgUid.text.trim()
            }
            config.name = cfgUname.text.trim()
            close()
        }
    }
}
