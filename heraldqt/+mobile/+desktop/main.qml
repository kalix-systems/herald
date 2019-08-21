import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.3
import QtQuick.Controls 2.13
import QtQuick.Controls.Material 2.12
import Qt.labs.settings 1.0

ApplicationWindow {
    visible: true
    width: 900
    height: 640
    title: qsTr("Herald")
    id: root

    SplitView {
        id: rootSplitView
        anchors.fill: parent
        Layout.fillWidth: true
        Layout.fillHeight: true
        orientation: Qt.Horizontal

        /// Contacts view for the desktop client, in DesktopContacts.qml
        DesktopContacts {
            id: contacts
        }

        /// placeholder element
        Rectangle {
            id: placeholder
            color: "lightblue"
            Text {
                text: "View 1"
                anchors.centerIn: parent
            }
        }
    }

    /// signals autoscaling behavior

    //SplitView {
    // }

    /// bottom border... this is a documented solution

    //    GridLayout {

    //        id: responsiveGrid
    //        width: parent.width
    //        height: parent.height
    //        columns: 2
    //        Layout.fillWidth: true
    //        Layout.fillHeight: true

    //       //Column {
    ////            id: contactCol
    ////            width: 300
    ////            height: parent.height
    ////            visible: true
    ////            Layout.fillHeight: true

    ////            SplitView {

    ////                    clip: true
    ////                    id: contactColumn
    ////                    width: parent.width
    ////                    height: parent.height
    ////                    orientation: Qt.Vertical

    ////                    handle:  Rectangle {

    ////                        implicitWidth: 4
    ////                        implicitHeight: 4
    ////                        Rectangle {
    ////                            x: parent.width / 2.1
    ////                            implicitWidth: parent.width / 12
    ////                            implicitHeight: 4
    ////                            radius : 100
    ////                        color: "black"
    ////                        }
    ////                        color: SplitHandle.pressed ? "#878787"
    ////                            : (SplitHandle.hovered ? Qt.lighter("#c7c7c7", 1.1) : "#c7c7c7")
    ////                    }

    ////                    ScrollView {

    ////                        id: row
    ////                        Layout.alignment: Qt.AlignLeft | Qt.AlignTop
    ////                        Layout.fillWidth: true

    ////                        ListView {
    ////                            id: listView
    ////                            clip: true
    ////                            width: 110
    ////                            height: 160
    ////                            interactive: true
    ////                            transformOrigin: Item.Center
    ////                            delegate: Item {
    ////                                width: 80
    ////                                height: 40
    ////                                Row {
    ////                                    id: row1
    ////                                    spacing: 10
    ////                                    Rectangle {
    ////                                        width: 40
    ////                                        height: 40
    ////                                        color: colorCode
    ////                                    }

    ////                                    Text {
    ////                                        text: name
    ////                                        anchors.verticalCenter: parent.verticalCenter
    ////                                        font.bold: true
    ////                                    }
    ////                                }
    ////                            }
    ////                            model: ListModel {
    ////                                ListElement {
    ////                                    name: "Grey"
    ////                                    colorCode: "grey"
    ////                                }

    ////                                ListElement {
    ////                                    name: "Red"
    ////                                    colorCode: "red"
    ////                                }

    ////                                ListElement {
    ////                                    name: "Blue"
    ////                                    colorCode: "blue"
    ////                                }

    ////                                ListElement {
    ////                                    name: "Green"
    ////                                    colorCode: "green"
    ////                                }
    ////                            }
    ////                        }
    ////                    }

    ////                    ScrollView {
    ////                        clip:  true
    ////                        id: row3
    ////                        Layout.alignment: Qt.AlignLeft | Qt.AlignTop
    ////                        Layout.rowSpan: 1
    ////                        Layout.fillWidth: true
    ////                        ListView {
    ////                            clip: true
    ////                            id: listView1
    ////                            width: 110
    ////                            height: 160
    ////                            delegate: Item {
    ////                                width: 80
    ////                                height: 40
    ////                                Row {
    ////                                    id: row2
    ////                                    spacing: 10
    ////                                    Rectangle {
    ////                                        width: 40
    ////                                        height: 40
    ////                                        color: colorCode
    ////                                    }

    ////                                    Text {
    ////                                        text: name
    ////                                        anchors.verticalCenter: parent.verticalCenter
    ////                                        font.bold: true
    ////                                    }
    ////                                }
    ////                            }
    ////                            model: ListModel {
    ////                                ListElement {
    ////                                    name: "Grey"
    ////                                    colorCode: "grey"
    ////                                }

    ////                                ListElement {
    ////                                    name: "Red"
    ////                                    colorCode: "red"
    ////                                }

    ////                                ListElement {
    ////                                    name: "Blue"
    ////                                    colorCode: "blue"
    ////                                }

    ////                                ListElement {
    ////                                    name: "Green"
    ////                                    colorCode: "green"
    ////                                }
    ////                            }
    ////                        }
    ////                    }
    ////                }
    ////            }
    ////}

    //       Column {
    //            id: chatCol
    //        }
    //    }
}
