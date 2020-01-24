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

        property var memberData: UserMap.get(model.memberId)
        PlatonicRectangle {
            boxTitle: memberId.name
            boxColor: memberId.userColor
            picture: memberId.profilePicture

            labelComponent: Entity.ContactLabel {
                displayName: memberId.name
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
