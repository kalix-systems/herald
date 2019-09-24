import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0

Page {
 header: UtilityBar {
   buttonSpacing: 30
   marginWidth: QmlCfg.margin
   iconDimLarge: 45
   iconDimSmall: 30
   toolBarHeight: 70
   secondary: QmlCfg.palette.secondaryColor
  }

  Button {
      id: composeButton
       height: 60
       width: height
      anchors {
          right: parent.right
          bottom: parent.bottom
          margins: 20
      }
      background: Rectangle {
        radius: composeButton.height
        color: QmlCfg.palette.secondaryColor
        anchors.fill: parent
        Image {
            source: "plus-icon.svg"
            sourceSize: Qt.size(48,48)
            anchors.fill: parent
        }
     }
  }

}
