import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "qrc:/imports"
import QtGraphicalEffects 1.0
import "../../common" as Common
import "qrc:/imports/Entity" as Av
import "qrc:/imports/js/utils.mjs" as Utils

Popup {
    id: moreInfoPopup
    property var convoMembers
    property var messageData

    height: root.height
    width: root.width
    anchors.centerIn: parent

    background: Rectangle {
        id: background
        color: CmnCfg.palette.white
    }

    ListView {
        height: contentHeight
        width: parent.width
        anchors.top: parent.top
        model: convoMembers
        highlightFollowsCurrentItem: false
        currentIndex: -1
        delegate: Item {
            height: visible ? CmnCfg.convoHeight : 0
            width: parent.width
            visible: memberData.userId !== Herald.config.configId
            property var memberData: model
            Common.PlatonicRectangle {

                boxTitle: memberData.name
                boxColor: memberData.color
                picture: Utils.safeStringOrDefault(memberData.picture, "")
                color: CmnCfg.palette.lightGrey
                labelComponent: Av.ConversationLabel {
                    contactName: memberData.name
                    lastBody: "@" + memberData.userId
                    labelColor: CmnCfg.palette.black
                    secondaryLabelColor: CmnCfg.palette.darkGrey
                    labelFontSize: CmnCfg.entityLabelSize
                }
                MouseArea {
                    id: hoverHandler
                }

                states: []
            }
        }
    }
}
