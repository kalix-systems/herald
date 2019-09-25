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
        width: QmlCfg.padding
    }

    Column {
        id: textMessageCol
        focus: true
        spacing: QmlCfg.padding
        topPadding: QmlCfg.padding
        anchors {
            right: parent.right
            left: parent.left
        }

        Repeater {
            id: chatListView
            anchors.fill: parent
            model: convModel

            delegate: Column {
                readonly property bool outbound: author === config.configId
                // this is where scroll bar position needs to be set to instantiate in the right location
                Component.onCompleted: chatScrollBar.position = 1.0

                // column is most correct to resize for extra content
                anchors {
                    // This is okay as a ternary, the types are enforced by QML.
                    right: outbound ? parent.right : undefined
                    left: !outbound ? parent.left : undefined
                    rightMargin: QmlCfg.margin
                    leftMargin: QmlCfg.margin
                }
                rightPadding: QmlCfg.margin

                //NOTE: see chat bubble form
                CVUtils.ChatBubbleForm {
                    messageText: body
                    additionalContent: ""
                    contentArgs: {
                        return {

                        }
                    }
                    // This is okay as a ternary, the types are enforced by QML.
                    bubbleColor: outbound ? QmlCfg.palette.tertiaryColor : QmlCfg.avatarColors[convModel.color]
                } //bubble
            } //bubble wrapper
        } // Repeater
    } //singleton Col
} // flickable
