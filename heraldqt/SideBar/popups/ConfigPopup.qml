import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "../../common" as Common
import "../../common/utils.mjs" as Utils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// RS: Rusts job
// Factor Component: FC

// Note: we want this to be a seperate window.
// afaict: all notes here will be tossed
Popup {
    id: configPopup
    modal: true
    focus: true
    width: 300
    height: 300
    x: (root.width - width) / 2
    y: (root.height - height) / 2

    Column {

        /// RS: check with the server to prevent duplicate ID's
        /// command line login and boot?!
        TextField {
            id: cfgUid
            enabled: !config.config_id
            property bool userIdValid: true
            placeholderText: enabled ? "Enter UID " : config.config_id
        }

        // RS and HERE : length constraint
        TextField {
            id: cfgUname
            property bool usernameValid: true
            placeholderText: "Enter Username"
        }

        // RS and HERE : file type constraints : *.jpg,*jpeg,*.png
        FileDialog {
            id: cfgPfp
            property bool pfpValid: true
            onSelectionAccepted: {
                config.profile_picture = fileUrl
                print("set to", fileUrl)
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
            // TS: this should be checked in ts.
            if (!!!config.config_id) {
                config.config_id = cfgUid.text.trim()
            }
            config.name = cfgUname.text.trim()
            close()
        }
    }
}
