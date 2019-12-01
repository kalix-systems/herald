import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.12
import "qrc:/common" as Common
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports" as Imports

Rectangle {
    id: wrapper
    color: CmnCfg.palette.sideBarHighlightColor
    width: parent.width
    height: Math.max(textCol.height, 20)

    property color startColor
    property string opText: parent.opText
    property string opName: parent.opName

    Rectangle {
        id: verticalAccent
        anchors.right: wrapper.left
        height: wrapper.height
        width: CmnCfg.smallMargin / 4
        color: CmnCfg.palette.avatarColors[herald.users.colorById(
                                               ownedConversation.builderopAuthor)]
    }

    Imports.ButtonForm {
        id: exitButton
        anchors {
            //  margins: CmnCfg.smallMargin
            right: parent.right
            top: parent.top
        }
        source: "qrc:/x-icon.svg"
        scale: 0.8
        onClicked: {
            ownedConversation.builder.clearReply()
        }
    }

    ColumnLayout {
        id: textCol

        Label {
            id: sender
            text: herald.users.nameById(ownedConversation.builder.opAuthor)
            Layout.leftMargin: CmnCfg.smallMargin
            Layout.rightMargin: CmnCfg.smallMargin
            Layout.bottomMargin: CmnCfg.margin / 2
            Layout.topMargin: CmnCfg.margin / 2
            Layout.preferredHeight: CmnCfg.smallMargin
            font.bold: true
            color: CmnCfg.palette.avatarColors[herald.users.colorById(
                                                   ownedConversation.builder.opAuthor)]
        }

        TextMetrics {
            id: opTextMetrics
            text: ownedConversation.builder.opBody
            elideWidth: (wrapper.width - CmnCfg.smallMargin) * 2
            elide: Text.ElideRight
        }

        TextEdit {
            text: opTextMetrics.elidedText
            Layout.maximumWidth: wrapper.width - CmnCfg.smallMargin
            Layout.topMargin: CmnCfg.margin / 2
            Layout.leftMargin: CmnCfg.smallMargin
            Layout.rightMargin: CmnCfg.smallMargin
            Layout.bottomMargin: CmnCfg.smallPadding
            wrapMode: Text.WrapAtWordBoundaryOrAnywhere
            Layout.alignment: Qt.AlignLeft
            selectByMouse: true
            selectByKeyboard: true
            readOnly: true
            color: CmnCfg.palette.mainTextColor
        }

        Label {
            Layout.leftMargin: CmnCfg.smallMargin
            Layout.bottomMargin: CmnCfg.smallPadding
            Layout.topMargin: 0
            Layout.rightMargin: CmnCfg.smallMargin
            font.pixelSize: 10
            text: Utils.friendlyTimestamp(ownedConversation.builder.opTime)
            id: timestamp
            color: CmnCfg.palette.secondaryTextColor
        }
    }
}
