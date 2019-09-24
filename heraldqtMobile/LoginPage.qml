import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtGraphicalEffects 1.13
import LibHerald 1.0
import "./LoginPage" as LP
Page {



Component {
      id: registerNewDevicePage
  Page {
    LinearGradient {
        anchors.fill: parent
        gradient: Gradient {
            GradientStop {
                position: 0.0
                color: "lightblue"
            }
            GradientStop {
                position: 1.0
                color: Qt.darker("lightblue", 1.4)
            }
        }
    }

    ColumnLayout {
        anchors.fill: parent
        Item {
            //spacer
            Layout.fillHeight: true
        }
        Item {
            id: logo
        }
        Item {
            id: userIdField

        }
        Item {
            id: nextButton
        }
        Item {
            //spacer
            Layout.fillHeight: true
        }
      }
    }
  }

   LP.LoginLandingPage {
        id: llp
   }

    StackView {
        id: loginStackView
        anchors.fill: parent
        initialItem: llp
        pushEnter: Transition {
              PropertyAnimation {
                  property: "opacity"
                  from: 0
                  to:1
                  duration: 200
              }
          }
    }
}
