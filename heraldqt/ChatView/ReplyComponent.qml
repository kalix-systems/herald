import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13

Rectangle {
    //message displayed in the textEdit
    property string messageText: ""
    //color of the bubble proper
    property color bubbleColor
    //who the message is from
    property string from: ""
    // the width the text sits at without wrapping
    readonly property int naturalWidth:  Math.min(2*chatPane.width / 3, messageMetrics.width) + QmlCfg.margin

    id: bubble
    color: bubbleColor


    TextMetrics {
        id: messageMetrics
        text: messageText
        elideWidth: 140
    }

    radius: QmlCfg.radius

    height: bubbleText.height + who.height +  QmlCfg.margin

    Label {
        anchors {
            topMargin: QmlCfg.margin / 2
            leftMargin: QmlCfg.margin / 2
            left: bubble.left
            top: bubble.top
        }
        id: who
        text: from
    }

    TextEdit {
        id: bubbleText
        text: messageMetrics.text
        width: naturalWidth - QmlCfg.margin / 2

        wrapMode: TextEdit.Wrap
        selectByMouse: true
        selectByKeyboard: true
        readOnly: true
        anchors {
            margins: QmlCfg.margin / 2
            left: bubble.left
            top: who.bottom
            topMargin: 0
        }
    }

    onWidthChanged: {
        parent.prefferredWidth = bubbleText.width + 10
    }

    onHeightChanged: {
        parent.height = height
    }

}
