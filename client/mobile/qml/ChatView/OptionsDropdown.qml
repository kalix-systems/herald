import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import LibHerald 1.0
import "qrc:/imports/ChatBubble"
import "qrc:/imports/EmojiKeyboard"
import "../Common" as CMN

Rectangle {
    property var cb
    signal deactivate
    signal activate
    property real boundHeight: 0
    width: parent.width
    visible: height != 0
    color: CmnCfg.palette.offBlack
    onActivate: boundHeight = 50
    onDeactivate: boundHeight = 0
    height: content.height

    property bool active: height > 0

    Behavior on height {
        NumberAnimation {
            easing.type: Easing.InOutQuad
            duration: 100
        }
    }

    Connections {
        target: chatList
        onCloseDropdown: {
            if (active)
                deactivate()
            cb.isSelected = false
        }
    }

    Item {
        id: content
        height: boundHeight
        visible: !emoKeysPopup.active

        anchors {
            right: parent.right
            left: parent.left
        }

        CMN.AnimIconButton {
            icon.color: CmnCfg.palette.white
            imageSource: "qrc:/x-icon.svg"
            anchors {
                left: parent.left
                leftMargin: CmnCfg.defaultMargin
                verticalCenter: parent.verticalCenter
            }
            onTapped: {
                deactivate()
                cb.isSelected = false
            }
        }

        Row {
            anchors {
                verticalCenter: parent.verticalCenter
                right: parent.right
                rightMargin: CmnCfg.defaultMargin
            }
            clip: true
            layoutDirection: Qt.RightToLeft
            spacing: CmnCfg.defaultMargin

            CMN.AnimIconButton {
                icon.color: CmnCfg.palette.white
                visible: false
                imageSource: "qrc:/download-icon.svg"
                anchors.verticalCenter: parent.verticalCenter
                onTapped: {

                }
            }

            CMN.AnimIconButton {
                icon.color: CmnCfg.palette.white
                imageSource: "qrc:/reply-icon.svg"
                anchors.verticalCenter: parent.verticalCenter
                onTapped: {
                    ownedMessages.builder.opId = msgId
                    deactivate()
                    cb.isSelected = false
                }
            }
            CMN.AnimIconButton {
                imageSource: "qrc:/emoticon-icon.svg"
                icon.color: CmnCfg.palette.white
                anchors.verticalCenter: parent.verticalCenter
                onTapped: {
                    deactivate()
                    cb.isSelected = false
                    emoKeysPopup.active = true
                    emojiPopup.open()
                }
            }

            CMN.AnimIconButton {
                imageSource: "qrc:/info-icon.svg"
                anchors.verticalCenter: parent.verticalCenter
                visible: !bubbleLoader.isAux
                icon.color: CmnCfg.palette.white
                onTapped: {
                    mainView.push(cb.infoPage)
                    deactivate()
                    cb.isSelected = false
                }
            }
        }

        Popup {
            id: emojiPopup
            parent: chatListView
            width: parent.width
            height: reactPopup.height
            anchors.centerIn: parent
            property alias reactPopup: emoKeysPopup
            background: Item {}
            modal: true

            onClosed: {
                reactPopup.active = false
            }
            Loader {
                id: emoKeysPopup
                active: false
                height: active ? CmnCfg.units.dp(200) : 0
                width: parent.width
                sourceComponent: EmojiPicker {
                    id: emojiPicker
                    horizontal: true

                    Component.onCompleted: {
                        emojiPicker.send.connect(function (emoji) {
                            ownedMessages.addReaction(index, emoji)
                        })
                    }
                }
            }
        }
    }
}
