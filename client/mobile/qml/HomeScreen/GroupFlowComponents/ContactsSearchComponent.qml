import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./../Controls"
import "../../Common"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils

Column {
    topPadding: CmnCfg.units.dp(24)
    Component.onCompleted: Herald.usersSearch.refresh()
    width: mainView.width - CmnCfg.megaMargin * 2

    TextArea {
        id: groupSelectText
        leftPadding: 0
        placeholderText: qsTr("Add members")
        onTextChanged: {
            Herald.usersSearch.filter = groupSelectText.text
            contactPopup.popup.open()
        }
    }

    Rectangle {
        id: groupSelectTextUnderline
        height: 1
        width: parent.width
        color: "black"
    }

    ComboBox {
        id: contactPopup
        model: Herald.usersSearch
        width: parent.width
        anchors.horizontalCenter: parent.horizontalCenter
        height: 1
        leftPadding: CmnCfg.defaultMargin

        background: Rectangle {
            visible: false
        }

        indicator: Rectangle {
            visible: false
        }
        delegate: Rectangle {
            property var contactData: model
            height: visible ? entityBlock.height : 0 //CmnCfg.units.dp(48) : 0
            width: parent.width
            visible: matched && contactData.userId !== Herald.config.configId
            anchors {
                rightMargin: CmnCfg.units.dp(12)
                leftMargin: CmnCfg.units.dp(12)
            }

            EntityBlock {
                id: entityBlock
                entityName: contactData.name
                subLabelText: '@' + contactData.userId
                color: CmnCfg.avatarColors[contactData.color]
                // TODO pfpPath

                anchors.leftMargin: CmnCfg.smallMargin
                anchors.rightMargin: CmnCfg.smallMargin
            }

            TapHandler {
                onTapped: {
                    Herald.conversationBuilder.addMember(contactData.userId)
                    contactPopup.popup.close()
                    Herald.usersSearch.clearFilter()
                    groupSelectText.text = ""
                }
            }
        }
    }

    FinalGroupList {}
}
