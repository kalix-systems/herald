import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./../Controls"
import "../../Common"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils

ListView {
    height: contentHeight
    width: parent.width
    model: Herald.conversationBuilder

    delegate: Rectangle {
        height: entityBlock.height
        width: parent.width

        EntityBlock {
            id: entityBlock
            entityName: Herald.users.nameById(memberId)
            subLabelText: '@' + memberId
            color: CmnCfg.palette.avatarColors[Herald.users.colorById(
                                                   memberId)]
            // TODO pfpPath
        }

        AnimIconButton {
            id: xIcon
            anchors.right: parent.right
            anchors.rightMargin: CmnCfg.defaultMargin / 2
            anchors.verticalCenter: parent.verticalCenter
            imageSource: "qrc:/x-icon.svg"
            onClicked : {
                Herald.conversationBuilder.removeMemberById(memberId)
            }
        }
    }
}
