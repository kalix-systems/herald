import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtGraphicalEffects 1.13
import LibHerald 1.0

Page {
    id: lndRoot
    readonly property alias backButton: backButton
    readonly property alias submitButton: submitButton
    readonly property string entryFieldText: unameField.text
    readonly property real heightUnit: lndRoot.height / 100.0
    readonly property real widthUnit: lndRoot.width / 100.0

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

   header:
       Item {
         height: 0
       Button {
        id: backButton
        text: "◄"
        font.pixelSize: 50
        background: Item {}
        anchors {
            left: parent.left
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
            id: userIdField
            Layout.preferredHeight: heightUnit * 30
            Layout.preferredWidth: widthUnit * 75
            Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
            Label {
                id: unameLabel
                text: "Username"
                font.pointSize: 16
                anchors.bottom: underscore.top
                anchors.margins: QmlCfg.smallMargin
            }

            Rectangle {
                id: underscore
                width: unameLabel.width * 2
                height: 1
                color: "black"
                anchors.bottom: unameField.top
                anchors.margins: QmlCfg.margin
            }

            TextField {
                id: unameField
                height: heightUnit * 5
                width: parent.width
                background: Rectangle {
                    radius: QmlCfg.radius / 2
                    anchors.fill: parent
                    color: "white"
                }
                placeholderText: "..."
            }

            Column {
                id: validatorColumn
                spacing: QmlCfg.margin
                anchors.margins: QmlCfg.margin * 3
                anchors.top: unameField.bottom
                Row {
                    spacing: QmlCfg.margin
                    Image {
                        source: "qrc:///../heraldqt/icons/mary.png"
                        width: 20
                        height: width
                    }
                    Label {
                        anchors.margins: QmlCfg.margin
                        text: "3 - 250 characters in length."

                    }
                }

                Row {
                    spacing: QmlCfg.margin
                    Image {
                        source: "qrc:///../heraldqt/icons/mary.png"
                        width: 20
                        height: width
                    }
                    Label {
                        text: "Available."
                    }
                }

                Row {
                    spacing: QmlCfg.margin
                    Image {
                        source: "qrc:///../heraldqt/icons/mary.png"
                        width: 20
                        height: width
                    }
                    Label {
                        text: "Only alphanumeric and punctuation characters."
                        wrapMode: Label.WordWrap
                        width:  widthUnit * 75
                    }
                }
            }
        }
        Item {
            Layout.preferredHeight: heightUnit * 30
            Layout.preferredWidth: widthUnit * 75
            Layout.alignment: Qt.AlignBottom | Qt.AlignHCenter
            Button {
                id: submitButton
                anchors.bottom: parent.bottom
                width: parent.width
                height: 50
                background: Rectangle {
                    radius: QmlCfg.radius
                    anchors.fill:parent
                    color: submitButton.pressed ?  Qt.darker("lightblue", 1.4) : Qt.darker("lightblue", 1.9)
                    Text {
                        anchors.centerIn: parent
                        color: "white"
                        text: "Submit"
                    }
                }
            }
        }
        Item {
            //spacer
            Layout.fillHeight: true
        }
    }
}
