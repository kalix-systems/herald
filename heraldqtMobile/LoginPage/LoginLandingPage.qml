import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtGraphicalEffects 1.13
import LibHerald 1.0


Page {
       id: llpRoot
       readonly property int heightUnit: llpRoot.height / 100
       readonly property int widthUnit: llpRoot.width / 100
       readonly property alias registerThisDevice: registerThisDevice
       readonly property alias registerWithExistingDevice: registerWithExistingDevice

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

       spacing: QmlCfg.margin

       Label {
           id: title
           Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
           Layout.preferredHeight: heightUnit * 10
           text: "HERALD"
           font.pointSize: 36
           font.bold: true
           font.letterSpacing: 10
       }

       Item {
           id: logo
           Layout.preferredHeight: heightUnit * 20
           Layout.preferredWidth: image.width
           Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
           Image {
               Layout.alignment: Qt.AlignTop
               id: image
               source: "qrc:///../heraldqt/icons/mary.png"
               width: heightUnit * 20
               height: width
           }
       }

       Button {
           id: registerThisDevice
           Layout.preferredHeight: heightUnit * 10
           Layout.preferredWidth: widthUnit * 75
           Layout.alignment: Qt.AlignHCenter
           background: Rectangle {
               radius: QmlCfg.radius
               height: 50
               width: parent.width
               color: registerThisDevice.pressed ?  Qt.darker("lightblue", 1.4) : Qt.darker("lightblue", 1.9)
               Text {
                   anchors.centerIn: parent
                   color: "white"
                   text: "Register New Device"
               }
           }
       }

       Row {
           Layout.preferredWidth: widthUnit*50
           Layout.alignment: Qt.AlignHCenter
           spacing: QmlCfg.smallMargin
           Rectangle {
               anchors.verticalCenter: parent.verticalCenter
               color: "black"
               height: 1
               width: parent.width / 2 - or.width
           }
           Text {
               id: or
               text: "OR"
               anchors.verticalCenter: parent.verticalCenter
           }
           Rectangle {
               anchors.verticalCenter: parent.verticalCenter
               color: "black"
               height: 1
               width: parent.width / 2 - or.width
           }
       }

       Button {
           id: registerWithExistingDevice
           Layout.preferredHeight: heightUnit*20
           Layout.preferredWidth: widthUnit*75
           Layout.alignment: Qt.AlignHCenter
           background:  Rectangle {
               radius: QmlCfg.radius
               height: 50
               width: parent.width
               color: registerWithExistingDevice.pressed ?  Qt.darker("lightblue", 1.4) : Qt.darker("lightblue", 1.9)
               Text {
                   anchors.centerIn: parent
                   color: "white"
                   text: "Register to Existing Device"
               }
           }
       }
       }
     }

