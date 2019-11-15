import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import "qrc:/imports/Avatar" as Av
import "qrc:/imports/js/utils.mjs" as Utils
import QtQml 2.13


Column {
    width: parent.width


    TextArea {
        id: groupSelectText
        leftPadding: 12
        placeholderText: "Add members"
        onTextChanged: {
            groupConvoMake.filter = groupSelectText.text
            contactPopup.popup.open()
        }
    }

    Rectangle {
        height: 2
        width: parent.width - CmnCfg.largeMargin
        anchors.horizontalCenter: parent.horizontalCenter
        color: "black"
    }


    ConversationBuilderUsers {
        id: groupConvoMake
    }

    ComboBox {
        id: contactPopup
            model: groupConvoMake
            width: parent.width - CmnCfg.largeMargin
            anchors.horizontalCenter: parent.horizontalCenter
            height: CmnCfg.smallMargin / 2
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
            visible: matched

            Common.PlatonicRectangle {
                color: "white"
                id: contactRectangle
                boxColor: contactData.color
                boxTitle: contactData.name
                picture: Utils.safeStringOrDefault(contactData.profilePicture, "")

                labelComponent: Av.ConversationLabel {
                    contactName: contactData.name
                    labelColor: CmnCfg.palette.secondaryColor
                    labelSize: 14
                    lastBody: "@" + contactData.userId
                }

               MouseArea {
                    anchors.fill: parent
                    onClicked: { groupMemberSelect.addMember(contactData.userId)
                        contactPopup.popup.close()
                        groupConvoMake.clearFilter()
                        groupSelectText.text = ""


                    }
             }
            }
        }
        }

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
 }


