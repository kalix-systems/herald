import QtQuick 2.14
import LibHerald 1.0
import QtQuick.Layouts 1.14

// NOTE: Here be dragons: this relies on dynamic scoping
// Don't use this outside of the ReplyBubble directory
//+ n more file count
Text {
    id: fileSurplus

    visible: replyWrapper.fileCount > 0
    text: "+ " + replyWrapper.fileCount + qsTr(" more")
    font.weight: Font.Light
    font.family: CmnCfg.chatFont.name
    color: CmnCfg.palette.darkGrey
    font.pixelSize: 12
}
