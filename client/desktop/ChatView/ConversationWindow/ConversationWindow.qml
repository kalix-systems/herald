import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick 2.13
import LibHerald 1.0
import "qrc:/imports/ChatBubble" as CB
import "qrc:/imports/Avatar"
import "." as CVUtils
import "qrc:/imports/js/utils.mjs" as Utils
import "../../SideBar/js/ContactView.mjs" as CUtils

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

    Connections {
        target: model
        onRowsInserted: {
            chatListView.contentY = chatListView.contentHeight
        }
    }

    delegate: Row {
        id: chatRow
        readonly property string proxyBody: body
        property string proxyReceiptImage: Utils.receiptCodeSwitch(
                                               receiptStatus)
        readonly property color userColor: CmnCfg.avatarColors[Herald.users.colorById(
                                                                   author)]
        readonly property string timestamp: Utils.friendlyTimestamp(
                                                insertionTime)

        readonly property bool outbound: author === Herald.config.configId

        readonly property string authName: outbound ? Herald.config.name : Herald.users.nameById(
                                                          author)
        readonly property string pfpUrl: outbound ? Herald.config.profilePicture : Herald.users.profilePictureById(
                                                        author)
        property alias highlight: bubbleActual.highlightItem
        property bool elided: body.length !== fullBody.length

        property var messageModelData: model
        anchors {
            right: outbound ? parent.right : undefined
            left: !outbound ? parent.left : undefined
            rightMargin: CmnCfg.margin
            leftMargin: CmnCfg.smallMargin
        }
        layoutDirection: outbound ? Qt.RightToLeft : Qt.LeftToRight

        spacing: CmnCfg.margin
        bottomPadding: isTail ? CmnCfg.mediumMargin / 2 : CmnCfg.smallMargin / 2
        topPadding: isHead ? CmnCfg.mediumMargin / 2 : CmnCfg.smallMargin / 2

        AvatarMain {
            iconColor: userColor
            initials: authName[0].toUpperCase()
            size: 28
            opacity: isTail ? 1 : 0
            anchors {
                bottom: parent.bottom
                margins: CmnCfg.margin
                bottomMargin: parent.bottomPadding
            }

            z: 10
            pfpPath: parent.pfpUrl
            avatarHeight: 28
        }

        CB.ChatBubble {
            id: bubbleActual
            convContainer: chatListView
            defaultWidth: chatListView.width * 0.66
            messageModelData: chatRow.messageModelData

            ChatBubbleHover {
                download: bubbleActual.imageAttach || bubbleActual.docAttach
            }
        }
    }
}
