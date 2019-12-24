import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common"
import "../SettingsMenu"
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils
import Qt.labs.platform 1.0

ToolBar {
    id: conversationViewHeader

    clip: true
    height: CmnCfg.toolbarHeight

    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }

    RowLayout {
        anchors.fill: parent
        Row {
            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: CmnCfg.defaultMargin
            spacing: CmnCfg.largeMargin

            Avatar {
                color: CmnCfg.palette.avatarColors[Herald.config.color]
                initials: Herald.config.name[0].toUpperCase()
                pfpPath: Utils.safeStringOrDefault(
                             Herald.config.profilePicture, "")
                diameter: CmnCfg.identityAvatarDiameter
                Layout.alignment: Qt.AlignCenter
                Layout.leftMargin: CmnCfg.units.dp(12)
            }

            Label {
                id: stateLabel
                text: qsTr("Conversations")
                font {
                    pixelSize: CmnCfg.headerTextSize
                    family: CmnCfg.labelFont.name
                    weight: Font.DemiBold
                }
                anchors.verticalCenter: parent.verticalCenter
                color: CmnCfg.palette.iconFill
            }
        }

        Row {
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: CmnCfg.units.dp(12)
            spacing: CmnCfg.units.dp(12)

            IconButton {
                id: searchButton
                color: CmnCfg.palette.iconFill
                imageSource: "qrc:/search-icon.svg"
            }

            IconButton {
                id: optionsButton
                color: CmnCfg.palette.iconFill
                imageSource: "qrc:/options-icon.svg"
                tapCallback: function () {
                    mainView.push(settingsMain)
                }
            }
        }
    }
}
