import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../../common" as Common
import "qrc:/imports/js/utils.mjs" as Utils
import ".././js/ContactView.mjs" as JS
import "../popups" as Popups
import "qrc:/imports/Entity" as Ent

/// --- displays a list of contacts
// TODO this seems to be dead code
ListView {
    id: contactList
    clip: true
    currentIndex: -1
    boundsBehavior: Flickable.StopAtBounds
    ScrollBar.vertical: ScrollBar {}

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

            labelComponent: Ent.ContactLabel {
                displayName: contactData.name
                username: contactData.userId
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
