import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import "popups/NewContactDialogue.mjs" as JS
import "../SideBar" as SBUtils

Component {
    id: groupSelectComponent


    Column {
        width: wrapperRect.width

    Rectangle {
        anchors.fill: parent
        id: wrapperRect
        anchors.verticalCenter: parent.verticalCenter
        color: QmlCfg.palette.mainColor


    Flow {

        id: groupFlow
        height: parent.height
        width: parent.width
        spacing: QmlCfg.smallMargin

        Repeater {
            Keys.enabled: true
            model: groupMemberSelect

          SBUtils.ContactBubble {

                text: name
                defaultColor: QmlCfg.avatarColors[bubbleColor]
                Layout.alignment: Qt.AlignLeft
                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                       parent.state == "" ? parent.state = "clickedstate" : parent.state = ""
                    }
                }



            }
          Keys.onPressed: {
              print("hi")
          }
        }

        TextArea {
            id: searchText

          placeholderText: searchLoader.searchPlaceholder

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
          }





          Common.ButtonForm {
              source: "qrc:/x-icon.svg"
              anchors.right: parent.right
              anchors.verticalCenter: parent.verticalCenter
              scale: 0.8
              onClicked: {
                  convoPane.state = ""
              }
          }



          onTextChanged: {
              if (contactsSearch) {
              Qt.callLater((text) => { contactsModel.filter = text }, searchText.text) }
              else {
                  Qt.callLater((text) => { conversationsModel.filter = text }, searchText.text)
              }
          }

          Keys.onReturnPressed: {
              if (convoPane.state == "newContactState") {
                  JS.insertContact(searchText, contactsModel, networkHandle, conversationsModel)
                  convoPane.state = ""
              }
          }






    }

    }



    }



}
}
