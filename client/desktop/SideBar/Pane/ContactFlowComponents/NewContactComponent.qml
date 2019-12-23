import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../../common" as Common
import QtQuick.Dialogs 1.3
import "../../popups/js/NewContactDialogue.mjs" as JS

Component {
    Rectangle {
        anchors.fill: parent
        color: CmnCfg.palette.offBlack

        ScrollView {
            id: titleText
            width: parent.width - CmnCfg.smallMargin / 2
            anchors.top: parent.top
            height: text.height
            anchors.topMargin: CmnCfg.smallMargin
            padding: 0
            TextArea {
                id: text
                leftPadding: 12
                color: CmnCfg.palette.white
                placeholderText: qsTr("Enter username or display name")
                width: parent.width - CmnCfg.megaMargin

                Keys.onReturnPressed: {
                    JS.insertContact(text, Herald.users)
                    sideBarState.state = ""
                }
            }
        }

        Rectangle {
            id: divider
            anchors.top: titleText.bottom
            height: 1
            width: parent.width - CmnCfg.megaMargin
            anchors.horizontalCenter: parent.horizontalCenter
            color: CmnCfg.palette.white
        }

        Rectangle {
            id: bigDivider
            anchors.top: divider.bottom
            anchors.topMargin: CmnCfg.defaultMargin
            height: 1
            width: parent.width
            color: CmnCfg.palette.white
        }
    }
}
