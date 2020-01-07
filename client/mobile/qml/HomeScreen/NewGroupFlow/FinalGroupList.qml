import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./../Controls"
import "../../Common"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Entity" as Entity
import "qrc:/imports/js/utils.mjs" as Utils

ListView {
    height: contentHeight
    width: parent.width
    model: Herald.conversationBuilder

    delegate: Rectangle {
        height: visible ? CmnCfg.convoHeight : 0
        width: parent.width

        PlatonicRectangle {
            boxTitle: memberName
            boxColor: memberColor
            picture: memberProfilePicture

            labelComponent: Entity.ContactLabel {
                displayName: Herald.users.nameById(memberId)
                username: memberId
            }
        }

        AnimIconButton {
            id: xIcon
            anchors.right: parent.right
            anchors.rightMargin: CmnCfg.defaultMargin / 2
            anchors.verticalCenter: parent.verticalCenter
            imageSource: "qrc:/x-icon.svg"
            onTapped: {
                Herald.conversationBuilder.removeMemberById(memberId)
            }
        }
    }
}
