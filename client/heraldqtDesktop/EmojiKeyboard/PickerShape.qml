import QtQuick 2.13
import QtQuick.Shapes 1.12

// the shape, used to mask the background object
// as specified by --window--
Item {
    property color pickerColor: "white"
    property int edgeRadius: 10

 Rectangle {
     id: body
     color: pickerColor
     anchors.fill: parent
     anchors.bottomMargin: carat.height
     radius: edgeRadius
 }

 Shape {
         id: carat
         width: 30
         height: 15
         anchors {
             left: parent.left
             leftMargin: 10
             top: body.bottom
         }

         ShapePath {
             fillColor: pickerColor
             strokeColor: "#00000000"
             strokeWidth: 0
             joinStyle: ShapePath.RoundCap
             startX: 0; startY: 0
             PathLine { x: 15;  y: 15  }
             PathLine { x: 30; y: 0 }
             PathLine { x: 0; y: 0 }
         }

     }

}

