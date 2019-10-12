import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import "../popups/js/NewContactDialogue.mjs" as JS
import "../../SideBar" as SBUtils
import "../../common/js/utils.mjs" as Utils

ListView {
    id: groupList

    delegate: Item {
        id: memberItem

        height: 55
        width: parent.width

        Rectangle {
            id: bgBox

            Common.Divider {
                color: QmlCfg.palette.secondaryColor
                anchor: parent.bottom
            }

            anchors.fill: parent

            color: QmlCfg.palette.mainColor
        }

        Common.Avatar {
            size: 45
            id: memberAvatar
            avatarLabel: contactsModel.nameById(memberId)
            labelGap: QmlCfg.smallMargin
            secondaryText: "@" + memberId
            colorHash: contactsModel.colorById(memberId)
            pfpUrl: Utils.safeStringOrDefault(contactsModel.profilePictureById(
                                                  memberId))
        }
    }
}
