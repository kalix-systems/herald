import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.12
import "../common" as Common
import LibHerald 1.0

 ScrollView {
     width: parent.width
     height: wrapperRow.height
    // ScrollBar.horizontal.policy: ScrollBar.AlwaysOn
     ScrollBar.vertical.policy: ScrollBar.AlwaysOff

     Row {
         id: wrapperRow
         height: 100
         Layout.margins: 10
         width: parent.width
         spacing: 5
         Repeater {
             id: imageRepeater
             model: builder
             delegate:


                 Rectangle {
                 height: 100
                 width: 100
                 border.color: "black"
                 border.width: 1
                 clip: true
                     Image {
                     id: image
                     anchors.fill: parent
                     anchors.margins: QmlCfg.smallMargin
                     source: "file:" + attachmentPath
                     fillMode: Image.PreserveAspectCrop
                     asynchronous: true

                     Button {
                         anchors.top: parent.top
                         anchors.right: parent.right
                         anchors.margins: QmlCfg.smallMargin
                         background: Rectangle {
                             color: "transparent"
                             width: x.width
                             height: x.height
                         }

                         Image {
                             id: x
                             source: "qrc:/x-icon.svg"
                             anchors.centerIn: parent
                             sourceSize: Qt.size(25, 25)
                         }
                         onClicked: {
                             builder.removeAttachmentByIndex(index)
                             print("hi")
                         }
                     }
                 }
         }
         }
     }
     bottomPadding: 5
 }
