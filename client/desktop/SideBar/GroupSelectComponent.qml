import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import "popups/NewContactDialogue.mjs" as JS
import "../SideBar" as SBUtils

Component {
    id: groupSelectComponent

    Rectangle {
        height: groupFlow.height
        width: parent.width
        id: wrapperRect
        color: QmlCfg.palette.mainColor

        Common.ButtonForm {
            id: backbutton
            source: "qrc:/back-arrow-icon.svg"
            anchors.left: parent.left
            anchors.top: parent.top
            scale: 0.7

        }


    Flow {

        topPadding: QmlCfg.smallMargin
        leftPadding: QmlCfg.smallMargin
       anchors.left: backbutton.right

        id: groupFlow
        width: parent.width
        spacing: QmlCfg.smallMargin / 2


        Repeater {
            id: contactBubbleRepeater
            Keys.enabled: true
            model: groupMemberSelect

          SBUtils.ContactBubble {

                text: displayName
                defaultColor: QmlCfg.avatarColors[groupMemberSelect.color(index)]

                Layout.alignment: Qt.AlignLeft
                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                       parent.state == "" ? parent.state = "clickedstate" : parent.state = ""
                    }
                }
                xButton.onClicked: {
                   // groupMemberSelect.removeMember(userId)
                }



            }
          Keys.onPressed: {
              print("hi")
          }
        }


        TextArea {
            id: searchText
            focus: true
            leftPadding: QmlCfg.smallMargin

          placeholderText: if (groupMemberSelect.count === 0) "Add people"
          else ""

          verticalAlignment: TextEdit.AlignVCenter
          background: Rectangle {
              color: QmlCfg.palette.mainColor
          }

          Keys.onPressed: {
              // NOTE: What is the first comparison doing?
              // this makes sure that returns and tabs are not evaluated
              if (event.key === Qt.Key_Return || event.key === Qt.Key_Tab){
                  event.accepted = true
              }
              if (event.key === Qt.Key_Backspace && text === "") {
                  groupMemberSelect.remove(groupMemberSelect.count - 1)
              }
          }




          onTextChanged: {
              if (contactsSearch) {
              Qt.callLater((text) => { contactsModel.filter = text }, searchText.text) }
              else {
                  Qt.callLater((text) => { conversationsModel.filter = text }, searchText.text)
              }
          }

    }

    }



    }



}
