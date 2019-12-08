import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./../Controls"
import "../../Common"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Avatar"
import "qrc:/imports/js/utils.mjs" as Utils

ListView {
    height: contentHeight
    width: parent.width
    model: herald.conversationBuilder

    delegate: Rectangle {
        height: CmnCfg.units.dp(48)
        width: parent.width

        AvatarMain {
            id: contactAvatar
            iconColor: CmnCfg.palette.avatarColors[herald.users.colorById(
                                                       memberId)]
            anchors.verticalCenter: parent.verticalCenter
            initials: Utils.initialize(herald.users.nameById(memberId))
            size: CmnCfg.units.dp(36)
            avatarHeight: CmnCfg.units.dp(36)

            anchors {
                right: parent.right
                left: parent.left
                leftMargin: CmnCfg.units.dp(12)
            }

            labelComponent: ConversationLabel {
                contactName: herald.users.nameById(memberId)
                labelColor: CmnCfg.palette.offBlack
                labelSize: 14
                lastBody: "@" + memberId
            }
        }

        IconButton {
            id: xIcon
            anchors.right: parent.right
            anchors.rightMargin: CmnCfg.margin / 2
            anchors.verticalCenter: parent.verticalCenter
            imageSource: "qrc:/x-icon.svg"
            tapCallback: function () {
                herald.conversationBuilder.removeMemberById(memberId)
            }
        }
    }
}
