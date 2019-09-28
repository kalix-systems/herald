import QtQuick 2.13
import QtQuick.Controls 2.12

Item {
    // most recent 20 emojis.
    property var mostRecent: []
    // enum skinTone { yellow, ... }
    property int skinTone: 0
    property color lowlight: "light gray"
    readonly property int categoryCount: 8
   // header and search bar
   Item {
       id: header
       height: 30
       anchors.top: parent.top
       anchors.topMargin: 10
       anchors.right: parent.right
       anchors.left: parent.left


       Rectangle {

           TextArea {
               placeholderText: "Search..."
               anchors.left: parent.left
               anchors.right: parent.right
               anchors.margins: 10
               anchors.verticalCenter: parent.verticalCenter
           }

           anchors {
               left: parent.left
               right: menu.left
               margins: 10
           }

           color: "#33000000" // transparent
           radius: 10
           border.color: "white"
           border.width: 0.5
           height: 25
       }

        Button {
            id: menu
            anchors.right: parent.right
            anchors.margins: 10
            anchors.verticalCenter: parent.verticalCenter
            height: 20
            width: 20
            background: Rectangle {
                id: bg
                radius: 5
                opacity: parent.pressed ? 1.0 : 0.0
                anchors.fill: parent
                color: lowlight
            }
            /// ToDo: bring up skin color dialog
            /// Maybe this should just be a skin color swatch, or
            /// and emoji
            Text {
                font.pixelSize: 20
                anchors.centerIn: parent
                font.bold: true
                text: "⋮"
            }
        }
   }

   Rectangle {
       width: parent.width
       height: 0.5
       color: "white"
       anchors.bottom: listView.top
   }


   Item {
       id: listView
       width: parent.width
       anchors {
           top: header.bottom
           bottom: footer.top
       }
     Flickable {
        anchors.fill: parent
        boundsBehavior: Flickable.StopAtBounds
        clip: true
        contentHeight: col.height
       Column {
           id: col
           spacing: 10
           Repeater {
                model: categoryCount
                Grid {
                    columns: 8
                    spacing: 2
                    Repeater {
                        model: 101
                        EmojiButton {}
                    }
                 }
              }
            }
          }
      }

   Item {
       id: footer
       width: parent.width
       height: 30
       anchors.bottom: parent.bottom
       anchors.bottomMargin: 20 // 10 + carat height

       Rectangle {
           width: parent.width
           height: 0.5
           color: "white"
       }

       // todo: these should be buttons
       // and not emojis
       Row {
           anchors.centerIn: parent
        Repeater {
            model: categoryCount
            EmojiButton {
               lowlight: lowlight
            }
        }
       }
   }
}
