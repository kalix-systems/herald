import QtQuick.Controls 2.14
import QtQuick.Layouts 1.12
import QtQuick 2.14
import LibHerald 1.0
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports/Avatar"
import "." as CVUtils
import "qrc:/imports/js/utils.mjs" as Utils
import "../../SideBar/js/ContactView.mjs" as CUtils
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import QtGraphicalEffects 1.0

ListView {
    id: chatListView
    property alias chatScrollBar: chatScrollBarInner
    property alias chatListView: chatListView

    //this should be in here and not in the bubble because conversation window
    //needs access to it, add a separate animation to mobile
    //do not move this back into foundation
    property NumberAnimation highlightAnimation: NumberAnimation {
        id: bubbleHighlightAnimation
        property: "opacity"
        from: 1.0
        to: 0.0
        duration: 600
        easing.type: Easing.InCubic
    }
    spacing: 0

    // disable these, we're handling them differently
    keyNavigationEnabled: false
    keyNavigationWraps: false

    // TODO this only clips because of highlight rectangles, figure out a way to
    // not use clip
    clip: true

    maximumFlickVelocity: 1500
    flickDeceleration: chatListView.height * 10

    onFlickStarted: focus = true

    highlightFollowsCurrentItem: false
    cacheBuffer: chatListView.height * 5

    Layout.maximumWidth: parent.width

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBarInner
        width: CmnCfg.padding

        policy: ScrollBar.AsNeeded
        hoverEnabled: true

        stepSize: 0.01
        minimumSize: 0.1
    }

    boundsBehavior: ListView.StopAtBounds
    boundsMovement: Flickable.StopAtBounds
    model: chatPane.ownedConversation

    Component.onCompleted: {
        model.setElisionLineCount(38)
        model.setElisionCharCount(38 * 40)
        model.setElisionCharsPerLine(40)
        positionViewAtEnd()

        // heuristic overshoot
        chatScrollBarInner.setPosition(2)
    }

    FileDialog {
        id: attachmentDownloader
        property string filePath
        selectFolder: true
        folder: StandardPaths.writableLocation(StandardPaths.DesktopLocation)
        onAccepted: Herald.utils.saveFile(filePath, fileUrl)
        selectExisting: false
    }

    delegate: Row {
        id: chatRow

        ListView.onAdd: {

            chatScrollBarInner.setPosition(3)
        }
        readonly property string proxyBody: body
        property string proxyReceiptImage: Utils.receiptCodeSwitch(
                                               receiptStatus)

        readonly property bool outbound: author === Herald.config.configId

        property alias highlight: bubbleActual.highlightItem
        property bool elided: body.length !== fullBody.length

        property var messageModelData: model

        //        onPositioningComplete: {
        //            if (index === count - 1)
        //                chatScrollBarInner.setPosition(1)
        //        }
        anchors {
            left: parent.left
            right: parent.right
        }

        //  layoutDirection: outbound ? Qt.RightToLeft : Qt.LeftToRight
        bottomPadding: 0 //CmnCfg.smallMargin / 2
        topPadding: 0 //CmnCfg.smallMargin / 2

        CB.ChatBubble {
            id: bubbleActual
            convContainer: chatListView
            defaultWidth: chatListView.width
            messageModelData: chatRow.messageModelData

            ChatBubbleHover {
                id: bubbleHoverHandler
                download: bubbleActual.imageAttach || bubbleActual.docAttach
                onEntered: bubbleActual.hoverHighlight = true
                onExited: bubbleActual.hoverHighlight = false
            }
        }
    }
}
