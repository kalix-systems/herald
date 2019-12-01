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
    model: herald.conversationBuilder

    delegate: Item {
        id: memberItem

        height: CmnCfg.convoHeight
        width: parent.width

        Common.PlatonicRectangle {
            color: CmnCfg.palette.paneColor
            id: memberRectangle
            boxColor: herald.users.colorById(memberId)
            boxTitle: herald.users.nameById(memberId)
            picture: Utils.safeStringOrDefault(herald.users.profilePictureById(
                                                   memberId), "")

            //no hover state
            states: []

            labelComponent: Av.ConversationLabel {
                contactName: herald.users.nameById(memberId)
                labelColor: CmnCfg.palette.secondaryColor
                labelSize: 14
                lastBody: "@" + memberId
            }

            Imports.ButtonForm {
                id: xIcon
                anchors.right: parent.right
                anchors.rightMargin: CmnCfg.largeMargin / 2
                anchors.verticalCenter: parent.verticalCenter
                source: "qrc:/x-icon.svg"
                onClicked: herald.conversationBuilder.removeMemberById(memberId)
            }
        }
    }
}
