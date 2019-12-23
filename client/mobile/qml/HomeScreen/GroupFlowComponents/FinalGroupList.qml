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
    model: Herald.conversationBuilder

    delegate: Rectangle {
        height: CmnCfg.units.dp(48)
        width: parent.width

        AvatarMain {
            id: contactAvatar
            backgroundColor: CmnCfg.palette.avatarColors[Herald.users.colorById(
                                                       memberId)]
            anchors.verticalCenter: parent.verticalCenter
            initials: Utils.initialize(Herald.users.nameById(memberId))
            size: CmnCfg.units.dp(36)
            avatarDiameter: CmnCfg.units.dp(36)

            anchors {
                right: parent.right
                left: parent.left
                leftMargin: CmnCfg.units.dp(12)
            }

            labelComponent: ConversationLabel {
                contactName: Herald.users.nameById(memberId)
                labelColor: CmnCfg.palette.offBlack
                labelFontSize: 14
                lastBody: "@" + memberId
            }
        }

        IconButton {
            id: xIcon
            anchors.right: parent.right
            anchors.rightMargin: CmnCfg.defaultMargin / 2
            anchors.verticalCenter: parent.verticalCenter
            imageSource: "qrc:/x-icon.svg"
            tapCallback: function () {
                Herald.conversationBuilder.removeMemberById(memberId)
            }
        }
    }
}
