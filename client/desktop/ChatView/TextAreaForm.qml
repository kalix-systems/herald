import QtQuick 2.4
import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import QtQuick.Layouts 1.13
import QtMultimedia 5.13
import "ChatTextAreaUtils.mjs" as CTUtils
import "../common" as Common

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// RS: Rusts job
// Factor Component: FC
Rectangle {
    id: textWrapperRect

    property var parentPage
    // height of the text area, computed in JS
    property int scrollHeight
    // height of the text content proper
    property int contentHeight: scrollView.contentHeight
    // object to forward keypresses to.
    property var keysProxy
    // the attatchments button
    property alias atcButton: attachmentsButton
    // the emoji Button
    property alias emojiButton: emojiButton
    // the text area
    property alias chatText: chatText
    // summy file Dialog
    property alias attachmentsDialogue: attachmentsDialogue
    // camera button
    property alias cameraButton: cameraButton
    property var replyId
    property string replyText: ""
    property var replyWidth

    color: QmlCfg.palette.mainColor
    clip: true

    height: containerCol.height

    Common.ButtonForm {
        id: attachmentsButton
        anchors.right: cameraButton.left
        anchors.bottom: parent.bottom
        source: "qrc:/plus-icon.svg"
    }

    Common.ButtonForm {
        id: emojiButton
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        source: "qrc:/emoji-icon.svg"
    }

    Common.ButtonForm {
        id: cameraButton
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.leftMargin: QmlCfg.margin
        source: "qrc:/camera-icon.svg"
    }

    // wrapper column so replies load
    Column {
        id: containerCol

        anchors {
            left: emojiButton.right
            right: attachmentsButton.left
            leftMargin: QmlCfg.smallMargin
            rightMargin: QmlCfg.smallMargin
        }
        topPadding: QmlCfg.smallMargin

        ColumnLayout {
            width: parent.width
            Layout.fillWidth: true

            Loader {
                property int wrapperWidth: containerCol.width
                property string messageText
                property color replyBubbleColor: "grey"
                id: replyLoader
                active: false
                source: "ReplyComponent.qml"
            }

            ScrollView {
                id: scrollView
                height: scrollHeight
                implicitWidth: containerCol.width
                focus: true

                TextArea {
                    id: chatText
                    background: Rectangle {
                        color: QmlCfg.palette.secondaryColor
                        anchors {
                            fill: parent
                            horizontalCenter: parent.horizontalCenter
                            bottom: parent.bottom
                        }
                        radius: QmlCfg.radius
                    }
                    selectionColor: QmlCfg.palette.tertiaryColor
                    selectByMouse: true
                    wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere
                    placeholderText: "Send a Message ..."
                    Keys.forwardTo: keysProxy
                    Keys.onEscapePressed: focus = false
                }
            }
        }
    }

    FileDialog {
        id: attachmentsDialogue
        folder: shortcuts.home
        onSelectionAccepted: {
            print("todo: attachments api")
        }
    }

    onStateChanged: {
        print(state)
    }

    states: [
        State {
            name: "replystate"
            PropertyChanges {
                target: replyLoader
                active: true
            }
        },
        State {
            name: "default"
            PropertyChanges {
                target: replyLoader
                active: false
            }
        }
    ]
}
