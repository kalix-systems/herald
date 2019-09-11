import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import "./ReplyComponent.mjs" as JS

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping

//NPB: just looks kind bad
Rectangle {
    //message displayed in the textEdit
    property string messageText: ""
    //color of the bubble proper
    property color replyBubbleColor
    //who the message is from
    property string from: ""
    // the width the text sits at without wrapping
    //NPB: same weird margin fuding
    readonly property int naturalWidth: JS.naturalWidth(
                                            chatPane.width,
                                            messageMetrics.width) + QmlCfg.margin
    // the width of the parent object that we either match or override
    property var uiContainer: {
        width: 0
    }

    color: bubbleColor
    height: col.height
    //NPB: same weird margin fudging
    width: Math.max(naturalWidth, uiContainer.width) + QmlCfg.smallMargin

    TextMetrics {
        id: messageMetrics
        text: messageText
        elideWidth: 140
    }

    radius: QmlCfg.radius

    Column {
        id: col
        spacing: 0
        Label {
            id: who
            text: from
        }

        TextEdit {
            id: bubbleText
            text: messageMetrics.text
            width: naturalWidth
            wrapMode: TextEdit.Wrap
            selectByMouse: true
            selectByKeyboard: true
            readOnly: true
        }
    }
}
