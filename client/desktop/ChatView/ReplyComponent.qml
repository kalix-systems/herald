import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12

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

    //who the message is from
    property string from: ""
    // the width the text sits at without wrapping
    //NPB: same weird margin fuding
   // readonly property int naturalWidth: heraldUtils.chatBubbleNaturalWidth(
               //                             wrapperWidth,
                                         //   messageMetrics.width) + QmlCfg.margin
    // the width of the parent object that we either match or override


    color: "light green"/*replyBubbleColor*/
    Layout.preferredHeight: col.height //+ QmlCfg.margin
    //NPB: same weird margin fudging
    Layout.preferredWidth: col.width + QmlCfg.margin
    Layout.minimumWidth: 100
    Layout.minimumHeight: 50

    TextMetrics {
        id: messageMetrics
        text: messageText
        elide: Text.ElideRight
        elideWidth: parent.width
    }

    radius: QmlCfg.radius


    ColumnLayout {
        id: col
        spacing: 0
        Label {
            id: who
            text: from
        }
       // width: 100
        Layout.fillHeight: true
        Layout.fillWidth: true
//       width: wrapperWidth
       Component.onCompleted: print(height)


        TextEdit {
            leftPadding: QmlCfg.smallMargin
            bottomPadding: QmlCfg.smallMargin
            topPadding: QmlCfg.smallMargin
            id: bubbleText
            text: messageMetrics.elidedText
            wrapMode: TextEdit.WrapAnywhere
            selectByMouse: true
            selectByKeyboard: true
            readOnly: true
        }
    }
}
