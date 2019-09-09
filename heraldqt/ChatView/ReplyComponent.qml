import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import "../common/utils.js" as Utils

Rectangle {
    //message displayed in the textEdit
    property string messageText: ""
    //color of the bubble proper
    property color bubbleColor
    //who the message is from
    property string from: ""
    // the width the text sits at without wrapping
    readonly property int naturalWidth:  Math.min(2*chatPane.width / 3, messageMetrics.width) + QmlCfg.margin
    // the width of the parent object that we either match or override
    property var uiContainer: { width: 0}

    id: replyBubble
    color: bubbleColor
    height: col.height
    width: Math.max(naturalWidth , uiContainer.width) + QmlCfg.margin / 2

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
