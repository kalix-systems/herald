import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtGraphicalEffects 1.13
import LibHerald 1.0
import "LoginPage" as LP

Page {
   id: loginPage

   LP.LoginLandingPage {
        id: llp
        // button of the same name
        registerThisDevice {
            onClicked: {
                loginStackView.push(lnd);
            }
        }
        // button of the same name
        registerWithExistingDevice {
            onClicked: {
                loginStackView.push(lnd);
            }
        }
   }

  LP.LoginNewDevice {
       id: lnd
       backButton {
         onClicked: loginStackView.pop();
       }
       submitButton {
         onClicked: {
               //todo: ease a transition in here
               heraldState.setConfigId(entryFieldText.trim())
           }
       }
   }

    StackView {
        id: loginStackView

        anchors {
            top: parent.top
            right: parent.right
            left: parent.left
            bottom: parent.bottom
        }

        initialItem: llp
    }


    footer : Button {
            id: footerLink
            implicitHeight: 50
            background: Rectangle {
                height: parent.height
                color: Qt.darker("lightblue", 2.5)
                Text {
                    color: "white"
                    text: "Terms of Service ➤"
                    anchors.centerIn: parent
            }
         }
      }
}
