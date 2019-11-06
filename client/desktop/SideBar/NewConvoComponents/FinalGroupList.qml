import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import "../popups/js/NewContactDialogue.mjs" as JS
import "../../SideBar" as SBUtils
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports/Avatar" as Av

ListView {
    id: groupList

    delegate: Item {
        id: memberItem
        height: CmnCfg.convoHeight
        width: parent.width
        property string memberName: contactsModel.nameById(memberId)
        property int memberColor: contactsModel.colorById(memberId)
        property string memberPfp: contactsModel.profilePictureById(
                                       memberId)

        Rectangle {
            id: bgBox

            Common.Divider {
                color: CmnCfg.palette.secondaryColor
                bottomAnchor: parent.bottom
            }

            anchors.fill: parent

            color: CmnCfg.palette.mainColor
        }


        Av.AvatarMain {
            anchors.fill: parent
            id: memberAvatar
            iconColor: CmnCfg.avatarColors[memberColor]
            initials: memberName[0].toUpperCase()
            pfpPath: Utils.safeStringOrDefault(memberPfp)
            anchors {
                margins: 6
            }

            labelComponent: Av.ConversationLabel {
                contactName: memberName
                lastBody: "@" + memberId
            }
        }
    }
}
