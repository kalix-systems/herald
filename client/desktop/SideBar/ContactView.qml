import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import "../common" as Common
import "../common/js/utils.mjs" as Utils
import "./js/ContactView.mjs" as JS
import "popups" as Popups

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

        // This ternary is okay, types are enforced by QML
        height: visible ? CmnCfg.convoHeight : 0
        width: parent.width
        visible: matched

        Common.PlatonicRectangle {
            id: contactRectangle
            boxColor: contactsModel.color(index)
            boxTitle: contactsModel.name(index)
            isContact: true

            MouseArea {
                id: hoverHandler
                hoverEnabled: true
                z: CmnCfg.overlayZ
                anchors.fill: parent
                onClicked: {
                    if (convoPane.state == "newGroupState") {
                        groupMemberSelect.addMember(userId)
                    }
                }
            }
        }
    }
} // BNOTE: Oh Christmas Tree, Oh Christmas Tree// This is pretty factored out already i'm not sure how to fix
