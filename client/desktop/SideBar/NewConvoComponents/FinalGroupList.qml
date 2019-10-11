import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import "../popups/NewContactDialogue.mjs" as JS
import "../../SideBar" as SBUtils
import "../../common/utils.mjs" as Utils

ListView {
    id: groupList

    delegate: Item {
        id: memberItem

        //PAUL: Demagic this number
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
            //PAUL: Demagic this number
            size: 45
            id: memberAvatar
            avatarLabel: memberDisplayName
            labelGap: QmlCfg.smallMargin
            secondaryText: "@" + memberId
            colorHash: memberColor
            pfpUrl: Utils.safeStringOrDefault(memberProfilePicture)
        }
    }
}
