import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import QtQuick 2.14
import LibHerald 1.0
import "../Common"
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils
import Qt.labs.platform 1.1

RowLayout {
    anchors.fill: parent

    Row {
        Layout.alignment: Qt.AlignLeft
        Layout.leftMargin: CmnCfg.units.dp(12)
        spacing: CmnCfg.units.dp(16)
        Avatar {
            color: CmnCfg.palette.avatarColors[Herald.config.color]
            initials: Herald.config.name[0].toUpperCase()
            pfpPath: Utils.safeStringOrDefault(Herald.config.profilePicture, "")
            size: CmnCfg.units.dp(24)
            anchors.verticalCenter: parent.verticalCenter
            Layout.leftMargin: CmnCfg.units.dp(12)
        }

        Label {
            id: stateLabel
            text: qsTr("Conversations")
            font: CmnCfg.headerFont
            anchors.verticalCenter: parent.verticalCenter
            color: CmnCfg.palette.iconFill
        }
    }

    Row {
        Layout.alignment: Qt.AlignRight
        Layout.rightMargin: CmnCfg.units.dp(12)
        spacing: CmnCfg.units.dp(12)

        AnimIconButton {
            id: searchButton
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/search-icon.svg"
        }

        AnimIconButton {
            id: optionsButton
            color: CmnCfg.palette.iconFill
            imageSource: "qrc:/options-icon.svg"
            onClicked: {
                mainView.push(settingsMain)
            }
        }
    }
}
