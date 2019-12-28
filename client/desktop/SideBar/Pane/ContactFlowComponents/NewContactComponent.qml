import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../../common" as Common
import "qrc:/imports" as Imports
import QtQuick.Dialogs 1.3
import "../../popups/js/NewContactDialogue.mjs" as JS

Component {
    Rectangle {
        anchors.fill: parent
        color: CmnCfg.palette.offBlack

        // TODO Colleen got rid of a scrollview around this text entry
        // because it probably shouldn't scroll, is this sensible?
        Imports.BorderedTextField {
            id: text
            anchors {
                top: parent.top
                topMargin: CmnCfg.smallMargin
                horizontalCenter: parent.horizontalCenter
            }
            placeholderText: qsTr("Enter username or display name")
            width: parent.width - CmnCfg.megaMargin

            Keys.onReturnPressed: {
                JS.insertContact(text, Herald.users)
                sideBarState.state = ""
            }
        }

        Rectangle {
            id: bigDivider
            anchors.top: text.bottom
            anchors.topMargin: CmnCfg.defaultMargin
            height: 1
            width: parent.width
            color: CmnCfg.palette.white
        }
    }
}
