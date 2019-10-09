import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import "popups/NewContactDialogue.mjs" as JS
import "../SideBar" as SBUtils

Component {
    id: searchBarComponent

    TextArea {
      id: searchText
      height: QmlCfg.toolbarHeight

      placeholderText: searchLoader.searchPlaceholder
      color: QmlCfg.palette.mainTextColor
      verticalAlignment: TextEdit.AlignVCenter

      background: Rectangle {
          color: QmlCfg.palette.mainColor
          anchors.fill: parent
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
          height: 20
          anchors.right: parent.right
          anchors.verticalCenter: parent.verticalCenter
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



