import QtQuick 2.4
import QtQuick.Controls 2.13
import LibHerald 1.0
import "ChatTextAreaUtils.js" as CTUtils

Rectangle {

    property var parentPage
    // height of the text area, computed in JS
    property int scrollHeight
    // height of the text content proper
    property int contentHeight: scrollView.contentHeight
    // object to forward keypresses to.
    property var keysProxy
    // the attatchments button
    property alias atcButton: attachmentsButton
    // the text area
    property alias chatText: chatText

    color: QmlCfg.palette.mainColor
    clip: true
    height: scrollHeight + QmlCfg.margin / 2

    Button {
        id: attachmentsButton
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        height: 25
        width: height
        background: Image {
            source: "qrc:///icons/paperclip.png"
            height: width
            scale: 0.9
            mipmap: true
        }
    }

    ScrollView {
        id: scrollView
        height: scrollHeight
        focus: true

        anchors {
            left: parent.left
            right: attachmentsButton.left
            bottom: parent.bottom
        }

        TextArea  {
            id: chatText
            background: Rectangle {
                color: QmlCfg.palette.secondaryColor
                anchors {
                    fill: parent
                    horizontalCenter: parent.horizontalCenter
                    verticalCenter: parent.verticalCenter
                }
                radius: QmlCfg.radius
            }
            selectByMouse: true
            wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere
            placeholderText: "Send a Message ..."
            Keys.forwardTo: keysProxy
        }
    }
}
