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
import "qrc:/imports" as Imports
import "qrc:/imports/js/utils.mjs" as Utils

Column {
    topPadding: CmnCfg.units.dp(24)
    Component.onCompleted: Herald.usersSearch.refresh()
    // TODO megaMargin here shouldnot apply to FinalGroupList component
    width: mainView.width - CmnCfg.megaMargin * 2

    Imports.BorderedTextField {
        id: groupSelectText
        placeholderText: qsTr("Add members")
        onTextChanged: {
            Herald.usersSearch.filter = groupSelectText.text
            contactPopup.popup.open()
        }
        anchors.left: parent.left
        anchors.right: parent.right
        color: CmnCfg.palette.black
        borderColor: CmnCfg.palette.black
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
            height: visible ? CmnCfg.convoHeight : 0
            width: parent.width
            visible: matched && contactData.userId !== Herald.config.configId
            anchors {
                rightMargin: CmnCfg.units.dp(12)
                leftMargin: CmnCfg.units.dp(12)
            }

            PlatonicRectangle {
                id: contactRect
                boxTitle: contactData.name
                boxColor: contactData.color
                picture: contactData.profilePicture ? contactData.profilePicture : null

                labelComponent: ContactLabel {
                    displayName: contactData.name
                    username: contactData.userId
                }
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
