import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../../common" as Common
import "qrc:/imports/Avatar" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import QtQml 2.13

Column {
    width: parent.width

    TextArea {
        id: groupSelectText
        leftPadding: 12
        placeholderText: qsTr("Add members")
        onTextChanged: {
            herald.usersSearch.filter = groupSelectText.text
            contactPopup.popup.open()
        }
    }

    Rectangle {
        height: 1
        width: parent.width - CmnCfg.largeMargin
        anchors.horizontalCenter: parent.horizontalCenter
        color: "black"
    }

    ComboBox {
        id: contactPopup
        model: herald.usersSearch
        width: parent.width - CmnCfg.largeMargin
        anchors.horizontalCenter: parent.horizontalCenter
        height: CmnCfg.smallMargin / 2

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
            visible: matched && contactData.userId !== herald.config.configId

            Common.PlatonicRectangle {
                color: "white"
                id: contactRectangle
                boxColor: contactData.color
                boxTitle: contactData.name
                picture: Utils.safeStringOrDefault(contactData.profilePicture,
                                                   "")

                labelComponent: Av.ConversationLabel {
                    contactName: contactData.name
                    labelColor: CmnCfg.palette.offBlack
                    labelSize: 14
                    lastBody: "@" + contactData.userId
                }

                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                        //TODO: THIS WILL TAKE MODEL OWNED BY GLOBAL STATE
                        herald.conversationBuilder.addMember(contactData.userId)
                        contactPopup.popup.close()
                        herald.usersSearch.clearFilter()
                        groupSelectText.text = ""
                    }
                }
            }
        }
    }
    //component for selected group members
    FinalGroupList {}
}
