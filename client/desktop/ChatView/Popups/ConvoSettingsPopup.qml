import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "qrc:/imports" as Imports
import QtGraphicalEffects 1.0
import "../../common" as Common
import "qrc:/imports/Entity" as Entity
import "qrc:/imports/js/utils.mjs" as Utils
import QtQuick.Layouts 1.3
import QtQuick.Shapes 1.12

Popup {
    id: convoSettingsPopup
    property var convoData: parent.convoData
    property var contactMember: parent.convoMembers[0]

    padding: 0
    height: chatView.height
    width: chatView.width
    anchors.centerIn: parent
    onClosed: groupSettingsLoader.active = false

    background: Rectangle {
        id: background
        color: CmnCfg.palette.white
    }

    Imports.IconButton {
        anchors.right: parent.right
        anchors.rightMargin: CmnCfg.defaultMargin
        anchors.verticalCenter: header.verticalCenter
        icon.source: "qrc:/x-icon.svg"
        fill: CmnCfg.palette.white
        onClicked: {
            convoSettingsPopup.close()
            convoSettingsPopup.active = false
        }
        z: header.z + 1
    }

    Rectangle {
        id: header
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.leftMargin: 1
        anchors.right: parent.right
        height: CmnCfg.toolbarHeight + 1
        color: CmnCfg.palette.offBlack
        Label {
            id: headerLabel
            anchors.left: parent.left
            anchors.leftMargin: CmnCfg.smallMargin
            text: "Conversation settings"
            font.pixelSize: CmnCfg.headerFontSize
            color: CmnCfg.palette.white
            anchors.verticalCenter: parent.verticalCenter
            font.family: CmnCfg.labelFont.name
        }
    }
    Rectangle {
        anchors.right: header.left
        color: CmnCfg.palette.lightGrey
        width: 1
        height: CmnCfg.palette.toolbarHeight
    }

    Flickable {
        width: chatView.width
        anchors.top: header.bottom
        anchors.bottom: parent.bottom
        contentWidth: width
        contentHeight: wrapperCol.height
        clip: true
        ScrollBar.vertical: ScrollBar {}
        boundsBehavior: Flickable.StopAtBounds
        Column {
            id: wrapperCol
            width: parent.width - CmnCfg.smallMargin * 2
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: CmnCfg.smallMargin
            padding: CmnCfg.smallMargin

            Common.PlatonicRectangle {
                boxTitle: contactMember.name
                boxColor: contactMember.color
                picture: Utils.safeStringOrDefault(memberData.picture, "")
                color: CmnCfg.palette.white
                labelComponent: Entity.ConversationLabel {
                    contactName: contactMember.name
                    lastBody: "@" + contactMember.userId
                    labelColor: CmnCfg.palette.black
                    secondaryLabelColor: CmnCfg.palette.darkGrey
                    labelFontSize: CmnCfg.entityLabelSize
                }
                states: []
                MouseArea {
                    id: hoverHandler
                }
            }
        }
    }
}
