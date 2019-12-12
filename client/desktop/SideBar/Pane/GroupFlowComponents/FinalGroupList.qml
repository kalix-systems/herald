import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../../common" as Common
import "qrc:/imports/Avatar" as Av
import "qrc:/imports" as Imports
import "qrc:/imports/js/utils.mjs" as Utils
import QtQml 2.13

ListView {
    height: contentHeight
    width: parent.width
    model: Herald.conversationBuilder

    delegate: Item {
        id: memberItem

        height: CmnCfg.convoHeight
        width: parent.width

        Common.PlatonicRectangle {
            color: CmnCfg.palette.lightGrey
            id: memberRectangle
            boxColor: Herald.users.colorById(memberId)
            boxTitle: Herald.users.nameById(memberId)
            picture: Utils.safeStringOrDefault(Herald.users.profilePictureById(
                                                   memberId), "")

            //no hover state
            states: []

            labelComponent: Av.ConversationLabel {
                contactName: Herald.users.nameById(memberId)
                labelColor: CmnCfg.palette.offBlack
                labelSize: 14
                lastBody: "@" + memberId
            }

            Imports.ButtonForm {
                id: xIcon
                anchors.right: parent.right
                anchors.rightMargin: CmnCfg.largeMargin / 2
                anchors.verticalCenter: parent.verticalCenter
                source: "qrc:/x-icon.svg"
                onClicked: Herald.conversationBuilder.removeMemberById(memberId)
            }
        }
    }
}
