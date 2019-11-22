import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import ".././js/ContactView.mjs" as JS
import "../popups" as Popups
import "qrc:/imports/Avatar" as Av

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC

/// --- displays a list of contacts
ListView {
    id: contactList
    clip: true
    currentIndex: -1
    boundsBehavior: Flickable.StopAtBounds
    ScrollBar.vertical: ScrollBar {
    }

    delegate: Item {
        id: contactItem
        property var contactData: model

        height: visible ? CmnCfg.convoHeight : 0
        width: parent.width
        visible: matched

        Common.PlatonicRectangle {
            id: contactRectangle
            boxColor: contactData.color
            boxTitle: contactData.name
            picture: Utils.safeStringOrDefault(contactData.profilePicture, "")

            labelComponent: Av.ConversationLabel {
                contactName: contactData.name
                labelColor: CmnCfg.palette.secondaryColor
                labelSize: 14
                lastBody: "@" + contactData.userId
            }

            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: CmnCfg.overlayZ
                anchors.fill: parent
            }
        }
    }
}
