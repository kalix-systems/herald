import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import "../popups/js/NewContactDialogue.mjs" as JS
import "../../SideBar" as SBUtils
import "../../../foundation/js/utils.mjs" as Utils

ListView {
    id: groupList

    delegate: Item {
        id: memberItem

        height: CmnCfg.convoHeight
        width: parent.width

        Rectangle {
            id: bgBox

            Common.Divider {
                color: CmnCfg.palette.secondaryColor
                bottomAnchor: parent.bottom
            }

            anchors.fill: parent

            color: CmnCfg.palette.mainColor
        }

        Common.Avatar {
            size: CmnCfg.avatarSize
            id: memberAvatar
            avatarLabel: contactsModel.nameById(memberId)
            labelGap: CmnCfg.smallMargin
            secondaryText: "@" + memberId
            colorHash: contactsModel.colorById(memberId)
            pfpUrl: Utils.safeStringOrDefault(contactsModel.profilePictureById(
                                                  memberId))
        }
    }
}
