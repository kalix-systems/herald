import QtQuick 2.13
import QtQuick.Layouts 1.12

Component {
    id: multipleImageView
    Item {
        height: contentHeight
        width: contentWidth
        Row {
            anchors.fill: parent
            Item {
                id: self
                anchors.fill: parent
                anchors.margins: self.width * 0.08
                Rectangle {
                    id: outerItem
                    color: "black"
                    anchors.horizontalCenter: parent.horizontalCenter
                    anchors.verticalCenter: parent.verticalCenter
                    anchors.horizontalCenterOffset: self.width * 0.08
                    anchors.verticalCenterOffset: -self.width * 0.08
                    width: parent.width - labelText.width
                    height: parent.height
                }

                Rectangle {
                    z: 2
                    color: "gray"
                    anchors.horizontalCenter: parent.horizontalCenter
                    anchors.verticalCenter: parent.verticalCenter
                    anchors.horizontalCenterOffset: self.width * 0.04
                    anchors.verticalCenterOffset: -self.width * 0.04
                    width: parent.width
                    height: parent.height
                }

                Image {
                    source: imageSource[0]
                    width: parent.width
                    height: parent.height
                }
            }
            Item {
                Text {
                    id: labelText
                    anchors.verticalCenter: outerItem.verticalCenter
                    anchors.leftMargin: marginWidth
                    text: "+ " + contentSource.length
                    font.pixelSize: self.height * 0.2 * gu
                    font.bold: true
                }
                Text {
                    id: more
                    anchors.verticalCenter: outerItem.verticalCenter
                    anchors.topMargin: marginWidth
                    anchors.top: labelText.bottom
                    text: "more"
                    font.pixelSize: self.height * 0.2 * gu
                    font.bold: true
                }
            }
        }
    }
}
