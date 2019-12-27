import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../../common" as Common
import "qrc:/imports" as Imports
import "qrc:/imports/Entity" as Entity
import "qrc:/imports/js/utils.mjs" as Utils
import QtQml 2.13

Column {
    width: parent.width

    Imports.BorderedTextField {
        id: groupSelectText
        anchors.horizontalCenter: parent.horizontalCenter
        width: parent.width - CmnCfg.megaMargin
        color: CmnCfg.palette.white
        placeholderText: qsTr("Add members")
        onTextChanged: {
            Herald.usersSearch.filter = groupSelectText.text
            contactPopup.popup.open()
        }
    }

    ComboBox {
        id: contactPopup
        model: Herald.usersSearch
        width: parent.width - CmnCfg.megaMargin
        anchors.horizontalCenter: parent.horizontalCenter
        height: 1

        //this and indicator are invisible, we don't want the combo box
        //controls to be visible
        background: Rectangle {
            height: 0
            width: 0
            visible: false
        }

        indicator: Rectangle {
            height: 0
            width: 0
            visible: false
        }
        delegate: Item {
            id: contactItem
            property var contactData: model
            height: visible ? CmnCfg.convoHeight : 0
            width: parent.width
            visible: matched && contactData.userId !== Herald.config.configId

            Common.PlatonicRectangle {
                color: CmnCfg.palette.offBlack
                id: contactRectangle
                boxColor: contactData.color
                boxTitle: contactData.name
                picture: Utils.safeStringOrDefault(contactData.profilePicture,
                                                   "")

                labelComponent: Entity.ConversationLabel {
                    contactName: contactData.name
                    labelColor: CmnCfg.palette.white
                    labelFontSize: 14
                    lastBody: "@" + contactData.userId
                }
                states: []

                MouseArea {
                    id: hoverHandler
                    anchors.fill: parent
                    onClicked: {
                        Herald.conversationBuilder.addMember(contactData.userId)
                        contactPopup.popup.close()
                        Herald.usersSearch.clearFilter()
                        groupSelectText.text = ""
                    }
                }
            }
        }
    }
    //component for selected group members
    FinalGroupList {}
}
