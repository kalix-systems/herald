import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import QtQuick.Dialogs 1.3
import "popups/js/NewContactDialogue.mjs" as JS

Component {
    Rectangle {
        anchors.fill: parent
        color: CmnCfg.palette.paneColor

        ScrollView {
            id: titleText
            width: parent.width - CmnCfg.smallMargin / 2
            anchors.top: parent.top
            height: text.height
            anchors.topMargin: CmnCfg.smallMargin
            TextArea {
                id: text
                leftPadding: 12
                placeholderText: "Enter username or display name"

                Keys.onReturnPressed: {
                    JS.insertContact(text, contactsModel)
                }
            }
        }

        Rectangle {
            anchors.top: titleText.bottom
            id: divider
            height: 2
            width: parent.width - CmnCfg.largeMargin
            anchors.horizontalCenter: parent.horizontalCenter
            color: "black"
        }

        Rectangle {
            anchors.top: divider.bottom
            anchors.topMargin: CmnCfg.margin
            id: bigDivider
            height: 1
            width: parent.width
            color: CmnCfg.palette.secondaryTextColor
        }
    }
}
