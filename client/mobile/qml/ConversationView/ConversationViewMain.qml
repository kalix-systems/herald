import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./Controls"

Page {
    id: cvMainView
    header: CVHeader {}
    background: Rectangle {
        color: CmnCfg.palette.mainColor
    }

    // the body of this entire element
    // displays conversations
    ListView {
        id: cvListView
        clip: true
        boundsBehavior: ListView.StopAtBounds

        anchors.margins: CmnCfg.units.dp(12)
        spacing: CmnCfg.units.dp(16)

        anchors.fill: parent
        model: conversationsModel

        delegate: CVListItem {
            ownedMessages: Messages {
                conversationId: conversationId
            }
        }

        ScrollBar.vertical: ScrollBar {}
    }

    // floating pencil button to trigger
    // new message flow
    CVFloatingButton {

        anchors {
            bottom: parent.bottom
            right: parent.right
            margins: CmnCfg.units.dp(12)
        }

        iconSource: "qrc:/pencil-icon-black.svg"
    }
}
