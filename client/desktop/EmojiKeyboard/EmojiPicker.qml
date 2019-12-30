import QtQuick 2.13
import QtQuick.Controls 2.12
import QtGraphicalEffects 1.12
import LibHerald 1.0

Rectangle {
    id: maskShape
    property string modifier: ""
    property var caratCenter
    property var window
    signal send(string emoji, bool takesMod)
    signal close

    height: 250
    width: 280
    color: CmnCfg.palette.offBlack
    border.color: "#FFFFFF"

    PickerInterior {
        z: 2
        anchors {
            fill: parent
            centerIn: parent
        }
    }
    onSend: {
        const emoji_data = {
            "emoji": emoji,
            "takesMod": takesMod
        }

        var any = false

        for (var i = 0; i < CmnCfg.recentEmojis.length; i++) {
            any = CmnCfg.recentEmojis[i].emoji === emoji
            if (any)
                break
        }

        if (!any) {
            if (CmnCfg.recentEmojis.length === 20) {
                CmnCfg.recentEmojis.splice(-1, 1)
            }
            CmnCfg.recentEmojis.unshift(emoji_data)
        }
    }
}
