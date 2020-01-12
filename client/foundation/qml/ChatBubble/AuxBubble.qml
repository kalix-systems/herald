import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "./ReplyBubble"
import "../js/utils.mjs" as Utils
import "../Entity"

Rectangle {
    id: bubbleRoot

    property real defaultWidth
    property bool expanded: false
    property var auxData
    property var messageModelData
    property bool outbound: messageModelData.author === Herald.config.configId

    property alias highlightItem: bubbleHighlight
    readonly property color bubbleColor: CmnCfg.palette.lightGrey

    readonly property real maxWidth: defaultWidth * 0.72
    property string friendlyTimestamp: outbound ? Utils.friendlyTimestamp(
                                                      messageModelData.insertionTime) : Utils.friendlyTimestamp(
                                                      messageModelData.serverTime)

    property string timerIcon: expirationTime !== undefined ? Utils.timerIcon(
                                                                  expirationTime,
                                                                  insertionTime) : ""
    readonly property string receiptImage: outbound ? Utils.receiptCodeSwitch(
                                                          messageModelData.receiptStatus) : ""
    readonly property color authorColor: CmnCfg.avatarColors[messageModelData.authorColor]
    property string authorName: outbound ? "You" : Herald.users.nameById(
                                               messageModelData.author)

    property bool hoverHighlight: false
    property alias expireInfo: expireInfo
    property int bubbleIndex
    property bool moreInfo: false
    property bool aux: true
    property MouseArea hitbox

    Connections {
        target: appRoot.globalTimer
        onRefreshTime: {
            friendlyTimestamp = Utils.friendlyTimestamp(
                        messageModelData.insertionTime)
            timerIcon = (expirationTime !== undefined) ? (Utils.timerIcon(
                                                              expirationTime,
                                                              insertionTime)) : ""
            expireInfo.expireTime = (expirationTime
                                     !== undefined) ? (Utils.expireTimeShort(
                                                           expirationTime,
                                                           insertionTime)) : ""
        }
    }
    height: contentRoot.height
    width: defaultWidth

    color: CmnCfg.palette.white

    Rectangle {
        anchors.top: parent.top
        width: parent.width
        height: 1
        color: CmnCfg.palette.medGrey
    }

    Rectangle {
        anchors.bottom: parent.bottom
        width: parent.width

        height: 1
        color: CmnCfg.palette.medGrey
    }
    Avatar {
        id: avatar
        visible: false
        size: CmnCfg.chatAvatarSize
        anchors {
            left: parent.left
            top: parent.top
            margins: CmnCfg.smallMargin
        }

        z: contentRoot.z + 1
    }

    Highlight {
        id: bubbleHighlight
        color: CmnCfg.palette.darkGrey
        z: bubbleRoot.z + 1
    }

    Rectangle {
        id: accent
        anchors.top: parent.top
        anchors.bottom: parent.bottom

        width: CmnCfg.accentBarWidth

        color: CmnCfg.palette.medGrey
        anchors.left: avatar.right
        anchors.leftMargin: CmnCfg.smallMargin
    }

    Button {
        id: receipt
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.margins: CmnCfg.smallMargin

        icon.source: receiptImage
        icon.height: 16
        icon.width: 16
        icon.color: CmnCfg.palette.iconMatte
        padding: 0
        background: Item {}
    }

    BubbleExpireInfo {
        id: expireInfo
    }

    Column {
        id: contentRoot
        anchors.left: accent.right
        // all messages are un-expanded on completion
        Component.onCompleted: bubbleRoot.expanded = false

        spacing: CmnCfg.smallMargin
        topPadding: isHead ? CmnCfg.smallMargin : CmnCfg.smallMargin
        leftPadding: CmnCfg.smallMargin
        bottomPadding: isTail ? CmnCfg.defaultMargin : CmnCfg.smallMargin
        Text {
            text: friendlyTimestamp
            font.family: CmnCfg.chatFont.name
            font.italic: true
            font.pixelSize: 12
            color: CmnCfg.palette.darkGrey
            elide: Text.ElideRight
            width: bubbleRoot.maxWidth
        }
        GridLayout {

            Text {
                id: actionText
                text: authorName + Utils.auxString(auxData.code,
                                                   auxData.content)
                font.family: CmnCfg.chatFont.name
                font.italic: true
                Layout.maximumWidth: bubbleRoot.maxWidth
                wrapMode: Text.WrapAtWordBoundaryOrAnywhere
            }
        }

        Loader {
            id: imageClip
            active: auxData.code === 2
            sourceComponent: ReplyImageClip {
                imageSource: "file:" + auxData.content
            }
        }
        Loader {
            active: messageModelData.reactions.length > 0

            sourceComponent: BubbleReacts {}
        }
    }
}
