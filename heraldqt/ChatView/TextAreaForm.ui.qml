import QtQuick 2.4
import QtQuick.Controls 2.13
import LibHerald 1.0
import "ChatTextAreaUtils.js" as CTUtils

Rectangle {

    property var parentPage
    property int scrollHeight
    property int contentHeight: scrollView.contentHeight
    property TextArea chatText: chatText
    color: "white"
    height: contentHeight + QmlCfg.margin
    anchors {
        bottom: parentPage.bottom
        left: parentPage.left
        bottomMargin: QmlCfg.margin / 2
        rightMargin: QmlCfg.margin / 2
        margins: QmlCfg.margin / 2
    }

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
        onClicked: atcButtonPressedHandler
    }

    ScrollView {
        id: scrollView
        height: scrollHeight

        anchors {
            left: parent.left
            right: attachmentsButton.left
            bottom: parent.bottom
        }

        TextArea {
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
            selectByKeyboard: true
            selectByMouse: true
            wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere
            placeholderText: "Send a Message ..."
        }
    }
}
