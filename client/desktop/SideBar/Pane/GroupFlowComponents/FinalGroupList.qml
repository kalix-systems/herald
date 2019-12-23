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
            color: CmnCfg.palette.offBlack
            id: memberRectangle
            boxColor: memberColor
            boxTitle: memberName
            picture: memberProfilePicture

            //no hover state
            states: []

            MouseArea {
                id: hoverHandler
            }

            labelComponent: Av.ConversationLabel {
                contactName: memberName
                labelColor: CmnCfg.palette.white
                labelSize: 14
                lastBody: "@" + memberId
            }

            Imports.ButtonForm {
                id: xIcon
                anchors.right: parent.right
                anchors.rightMargin: CmnCfg.megaMargin / 2
                anchors.verticalCenter: parent.verticalCenter
                source: "qrc:/x-icon.svg"
                fill: CmnCfg.palette.lightGrey
                onClicked: Herald.conversationBuilder.removeMemberById(memberId)
            }
        }
    }
}
