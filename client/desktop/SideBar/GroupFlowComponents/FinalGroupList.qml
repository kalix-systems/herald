import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import "qrc:/imports/Avatar" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import QtQml 2.13

ListView {
    height: contentHeight
    width: parent.width
    model: groupMemberSelect

    delegate: Item {
    id: memberItem

    height: CmnCfg.convoHeight
    width: parent.width

    Common.PlatonicRectangle {
        color: CmnCfg.palette.paneColor
        id: memberRectangle
        boxColor: contactsModel.colorById(memberId)
        boxTitle: contactsModel.nameById(memberId)
        picture: Utils.safeStringOrDefault(contactsModel.profilePictureById(memberId), "")

        states: []

        labelComponent: Av.ConversationLabel {
            contactName: contactsModel.nameById(memberId)
            labelColor: CmnCfg.palette.secondaryColor
            labelSize: 14
            lastBody: "@" + memberId
        }


        Common.ButtonForm {
            id: xIcon
            anchors.right: parent.right
            anchors.rightMargin: CmnCfg.largeMargin / 2
            anchors.verticalCenter: parent.verticalCenter
            source: "qrc:/x-icon.svg"
            onClicked: groupMemberSelect.removeMemberById(memberId)
        }
     }


    }
}
