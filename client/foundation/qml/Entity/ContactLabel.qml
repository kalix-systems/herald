import QtQuick 2.12
import QtQuick.Layouts 1.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "qrc:/imports/js/utils.mjs" as JS

// Label showing display name and username for a contact
Column {
    // the group name or displayName of the conversation
    property string displayName
    property string username

    property color labelColor: CmnCfg.palette.black
    property int displayNameSize: CmnCfg.entityLabelSize
    property int usernameSize: CmnCfg.entitySubLabelSize

    height: CmnCfg.avatarSize
    spacing: CmnCfg.units.dp(3)

    Label {
        id: displayNameLabel
        font {
            family: CmnCfg.chatFont.name
            pixelSize: displayNameSize
            weight: Font.Medium
        }
        elide: "ElideRight"
        text: displayName
        color: labelColor
    }

    Label {
        id: usernameLabel
        font {
            family: CmnCfg.chatFont.name
            pixelSize: usernameSize
        }
        elide: "ElideRight"
        text: '@' + username
        color: labelColor
    }
}
