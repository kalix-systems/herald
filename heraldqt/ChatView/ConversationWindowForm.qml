import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "." as CVUtils
import "../common/utils.mjs" as Utils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// RS: Rusts job
// Factor Component: FC
Flickable {
    property alias chatScrollBar: chatScrollBar
    property alias chatListView: chatListView

    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds
    contentHeight: textMessageCol.height

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBar
        width: QmlCfg.margin
    }


    Column {
        id: textMessageCol
        focus: true
        spacing: QmlCfg.margin
        topPadding: QmlCfg.margin
        anchors {
            right: parent.right
            left: parent.left
        }



        Repeater {
            id: chatListView
            anchors.fill: parent
            model: messageModel


            delegate: Column {
                readonly property bool outbound: author === config.config_id


                //NPB: possibly not a column and just fix anchors
                // column is most correct to resize for extra content
                anchors {
                    right: if (outbound) {
                               parent.right
                           }
                    rightMargin: chatScrollBar.width + QmlCfg.margin
                    leftMargin: rightMargin
                }

                //NOTE: see chat bubble form. maybe handle this in TS:
                CVUtils.ChatBubbleForm {
                    messageText: body
                    additionalContent: ""
                    contentArgs: {
                        return {

                        }
                    }
                    bubbleColor: if (outbound) {
                                     QmlCfg.palette.tertiaryColor
                                 } else {
                                     QmlCfg.palette.secondaryColor
                                 }
                } //bubble
            } //bubble wrapper
        } // Repeater
    } //singleton Col
} // flickable


