import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.12

/// --- displays a list of contacts
Row {
    id : parent_row
    property string displayName: ""
    property string pfpUrl: ""
    property int colorHash: 0
    property int shapeEnum: 0 /// { individual, group ... }
    ///--- Circle with initial
    leftPadding: 10
    anchors.verticalCenter: parent.verticalCenter

    state: !pfpUrl ? "initial_avatar" : "image_avatar"

    states: [
        State {
            name: "initial_avatar"
            StateChangeScript {
              script: {
                  if (rowText) {
                      rowText.destroy()
                  }
                  fir.createObject(parent_row)
                  rowText.createObject(parent_row)
                  if (sec) {
                      sec.destroy();
                  }
               }
            }
        },
        State {
            name: "image_avatar"
            StateChangeScript {
                script: {
                    if (rowText) {
                        rowText.destroy()
                    }
                    sec.createObject(parent_row);
                    rowText.createObject(parent_row)
                    if (sec) {
                        sec.destroy();
                    }
                }
            }
        }
    ]

   Component {
       id: fir
    Rectangle {
        width: rowHeight - 10
        height: rowHeight - 10
        anchors.verticalCenter: parent.verticalCenter
        color:  QmlCfg.avatarColors[colorHash]
        radius: shapeEnum == 0 ? width : 0
        ///---- initial
        Text {
            text: qsTr(displayName[0].toUpperCase())
            font.bold: true
            color: "white"
            anchors.centerIn: parent
            font.pixelSize: parent.height - 5
        }
    }
   }

   Component {
       id: sec
       Image {
           width: rowHeight - 10
           height: rowHeight - 10
           source: "file:" + pfpUrl
       }
   }
    
  Component {
    id : rowText
    Text {
        text: displayName
        font.bold: true
        anchors.verticalCenter: parent.verticalCenter
    }
  }
    spacing: 10
}
