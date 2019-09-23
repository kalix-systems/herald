import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13

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

    Connections {
        target: appRoot
        onGsConversationIdChanged: {
            if (gsConversationId === undefined) {
                contactList.currentIndex = -1
            }
        }
    }

    ScrollBar.vertical: ScrollBar {
    }

    delegate: Item {
        id: contactItem
        // This ternary is okay, types are enforced by QML
        height: visible ? 60 : 0
        width: parent.width
        visible: matched

        Rectangle {
            id: bgBox
            anchors.fill: parent
            border.color: QmlCfg.palette.secondaryColor
        }

        Avatar {
            size: 50
            id: contactAvatar
            displayName: contactsModel.displayName(index)
            colorHash: color
            pfpUrl: Utils.safeStringOrDefault(profilePicture)
        }
    }
}
