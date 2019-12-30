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

    delegate: CB.ChatBubble {
        id: bubbleActual
        convContainer: chatListView
        defaultWidth: chatListView.width
        width: parent.width
        messageModelData: model
        ListView.onAdd: {
            chatScrollBarInner.setPosition(1.0)
            conversationItem.status = 0
        }
        bubbleIndex: index

        ChatBubbleHover {
            id: bubbleHoverHandler
            download: bubbleActual.imageAttach || bubbleActual.docAttach
            onEntered: {
                bubbleActual.hoverHighlight = true
                bubbleActual.expireInfo.visible = false
            }
            onExited: {
                if (reactPopup.active == true) {
                    bubbleActual.hoverHighlight = true
                }

                bubbleActual.hoverHighlight = false
                if (isHead)
                    bubbleActual.expireInfo.visible = true
            }
        }

        Popup {
            id: emojiMenu
            width: reactPopup.width
            height: reactPopup.height
            x: chatListView.width - width
            onClosed: {
                reactPopup.active = false
            }
            onOpened: {
                bubbleActual.hoverHighlight = true
            }

            y: {
                if (bubbleActual.y - chatListView.contentY > height) {
                    return -height
                }
                return CmnCfg.largeMargin * 2
            }

            Popups.EmojiPopup {
                id: reactPopup
                anchors.centerIn: parent
                isReactPopup: true
                x: chatListView.width - width

                z: convWindow.z + 1000
                onActiveChanged: {
                    if (!active)
                        emojiMenu.close()
                }

                anchors {
                    margins: CmnCfg.smallMargin
                }
            }
        }

        Loader {
            id: markReadLoader
            active: false
            Connections {
                target: root
                onActiveChanged: if (root.active) {
                                     ownedConversation.markRead(index)
                                 }
            }
        }

        Component.onCompleted: {
            markReadLoader.active = true
        }
    }
}
