import QtQuick.Controls 2.14
import QtQuick.Layouts 1.12
import QtQuick 2.14
import LibHerald 1.0
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports/Entity"
import "." as CVUtils
import "qrc:/imports/js/utils.mjs" as Utils
import "../../SideBar/js/ContactView.mjs" as CUtils
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import QtGraphicalEffects 1.0
import "../Popups" as Popups
import "qrc:/imports" as Imports

ListView {
    id: chatListView
    property alias chatScrollBar: chatScrollBarInner

    //this should be in here and not in the bubble because conversation window
    //needs access to it, add a separate animation to mobile
    //do not move this back into foundation
    property NumberAnimation highlightAnimation: NumberAnimation {
        id: bubbleHighlightAnimation
        property: "opacity"
        from: 0.4
        to: 0.0
        duration: 800
        easing.type: Easing.InCubic
    }

    spacing: 0
    // disable these, we're handling them differently
    keyNavigationEnabled: false
    keyNavigationWraps: false
    maximumFlickVelocity: 1500
    flickDeceleration: chatListView.height * 10

    onFlickStarted: focus = true

    highlightFollowsCurrentItem: false

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBarInner
        width: CmnCfg.smallMargin

        policy: ScrollBar.AsNeeded
        hoverEnabled: true

        stepSize: 0.01
        minimumSize: 0.1
    }

    boundsBehavior: ListView.StopAtBounds
    boundsMovement: Flickable.StopAtBounds
    model: chatPage.ownedConversation

    // Note: we load the list view from the bottom up to make
    // scroll behavior more predictable
    verticalLayoutDirection: ListView.VerticalBottomToTop

    // this is set to a higher value in `Component.onCompleted`
    // but is set to `0` here to improve initial load times
    cacheBuffer: 0

    Component.onCompleted: {
        model.setElisionLineCount(38)
        model.setElisionCharCount(38 * 40)
        model.setElisionCharsPerLine(40)
        chatScrollBarInner.setPosition(1.0)
        cacheBuffer = chatListView.height * 5
        if (chatListView.count === 0) {
            chatListView.height = chatListView.contentHeight
        }
    }

    FileDialog {
        id: downloadFileChooser
        selectFolder: true
        folder: StandardPaths.writableLocation(StandardPaths.DesktopLocation)
        onAccepted: ownedConversation.saveAllAttachments(index, fileUrl)
        selectExisting: false
    }

    delegate: Loader {
        id: bubbleLoader
        property var modelData: model
        sourceComponent: model.auxData.length === 0 ? msgBubble : auxBubble
        width: parent.width
        height: active ? item.height : undefined
        z: parent.z + 1

        property var highlightItem: active ? item.highlightItem : undefined
        Component.onCompleted: {
            if (root.active)
                ownedConversation.markReadById(model.msgId)
        }

        Connections {
            target: root
            // The additional check is because on deletion, root.active changes due to the popup and
            // the connections lives a little longer than the parent loader, causing it to attempt to mark read
            // an undefined msg id
            onActiveChanged: if (root.active && model.msgId !== undefined) {
                                 ownedConversation.markReadById(model.msgId)
                             }
        }

        Component {
            id: auxBubble

            ChatBubbleHover {
                id: hover
                childBubble: bubbleActual
                CB.AuxBubble {
                    id: bubbleActual
                    auxData: JSON.parse(model.auxData)
                    messageModelData: model
                    width: parent.width
                    defaultWidth: chatListView.width
                    bubbleIndex: index
                    hitbox: hover
                    ListView.onAdd: {
                        chatScrollBarInner.setPosition(1.0)
                    }
                    BubbleHoverButtons {
                        id: buttons
                    }
                    BubbleEmojiPopup {
                        id: emojiMenu
                    }
                }
            }
        }

        Component {
            id: msgBubble
            ChatBubbleHover {
                id: hover
                childBubble: bubbleActual
                CB.ChatBubble {

                    id: bubbleActual
                    convContainer: chatListView
                    defaultWidth: chatListView.width
                    width: parent.width
                    messageModelData: model
                    convoExpiration: conversationItem.expirationPeriod
                    ListView.onAdd: {
                        chatScrollBarInner.setPosition(1.0)
                    }
                    bubbleIndex: index
                    hitbox: hover
                    BubbleHoverButtons {
                        id: buttons
                    }
                    BubbleEmojiPopup {
                        id: emojiMenu
                    }
                }
            }
        }
    }
}
