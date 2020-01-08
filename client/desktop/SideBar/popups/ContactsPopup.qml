import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.13
import QtQuick.Window 2.2
import LibHerald 1.0
import "qrc:/imports"
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils

Window {
    id: settingsPopup
    minimumWidth: 350
    minimumHeight: 450

    Drawer {
        width: 0.33 * settingsPopup.width
        height: settingsPopup.height
        edge: Qt.RightEdge
        dragMargin: 0

        IconButton {
            fill: CmnCfg.palette.lightGrey
            source: "qrc:/x-icon.svg"
            anchors {
                top: parent.top
                right: parent.right
                margins: CmnCfg.smallMargin
            }
            onClicked: parent.close()
        }

        Flickable {
            anchors.fill: parent
        }
    }

    Page {
        anchors.fill: parent
        header: ToolBar {
            id: toolBar
            height: CmnCfg.toolbarHeight
            background: Rectangle {
                color: CmnCfg.palette.offBlack
            }

            RowLayout {
                anchors.fill: parent
                anchors.rightMargin: CmnCfg.defaultMargin
                anchors.leftMargin: CmnCfg.defaultMargin

                Label {
                    font: CmnCfg.headerFont
                    Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                    Layout.fillWidth: true
                    elide: Label.ElideRight
                    text: "Contacts"
                    color: CmnCfg.palette.white
                    topPadding: 1
                }

                IconButton {
                    id: searchButton
                    fill: CmnCfg.palette.lightGrey
                    source: "qrc:/search-icon.svg"
                }

                Item {
                    Layout.preferredWidth: CmnCfg.defaultMargin
                }

                IconButton {
                    id: settignsButton
                    fill: CmnCfg.palette.lightGrey
                    source: "qrc:/options-icon.svg"
                }
            }
        }

        RowLayout {
            id: rowLabel
            height: CmnCfg.toolbarHeight
            width: parent.width

            Item {
                Layout.preferredWidth: CmnCfg.avatarSize
            }

            Text {
                Layout.alignment: Qt.AlignLeft
                text: "Name"
            }
            Text {
                Layout.alignment: Qt.AlignLeft
                Layout.minimumWidth: CmnCfg.largeMargin
                text: "Trusted"
            }

            Text {
                Layout.minimumWidth: CmnCfg.largeMargin
                text: "Groups"
            }
        }

        ListView {
            id: tableView
            boundsBehavior: Flickable.StopAtBounds
            boundsMovement: Flickable.StopAtBounds
            anchors {
                top: rowLabel.bottom
                bottom: parent.bottom
                right: parent.right
                left: parent.left
            }
            model: Herald.users
            delegate: Rectangle {
                color: CmnCfg.palette.white
                width: settingsPopup.width
                height: row.height + 1
                Rectangle {
                    anchors {
                        right: parent.right
                        left: parent.left
                        top: parent.top
                    }
                    height: 1
                    color: CmnCfg.palette.black
                }

                RowLayout {
                    id: row
                    width: settingsPopup.width

                    Avatar {
                        Layout.alignment: Qt.AlignLeft
                        Layout.leftMargin: CmnCfg.defaultMargin
                        height: CmnCfg.avatarSize - CmnCfg.defaultMargin
                        pfpPath: Utils.safeStringOrDefault(
                                     model.profilePicture, "")
                        color: CmnCfg.avatarColors[model.color]
                        initials: Utils.initialize(name)
                    }

                    Column {
                        Layout.alignment: Qt.AlignLeft
                        spacing: CmnCfg.smallMargin / 2
                        Label {
                            font.bold: true
                            text: userId
                        }
                        Label {
                            text: "@" + name
                        }
                    }

                    IconButton {
                        Layout.alignment: Qt.AlignLeft
                        Layout.minimumWidth: CmnCfg.largeMargin
                        source: "qrc:/contacts-icon.svg"
                    }

                    Flow {
                        Layout.alignment: Qt.AlignCenter
                        Layout.preferredWidth: 85
                        Layout.preferredHeight: 45
                        spacing: CmnCfg.microMargin
                        Repeater {
                            model: [0, 0, 1, 1, 1, 1]
                            Image {
                                id: imagePorxy
                                source: "qrc:/plus-icon.svg"
                                width: 20
                                height: 20
                                Rectangle {
                                    visible: model.length > 6 && index == 5
                                    color: CmnCfg.palette.lightGrey
                                    anchors.fill: parent
                                    opacity: 0.5
                                    Text {
                                        color: CmnCfg.palette.white
                                        anchors.centerIn: parent
                                        text: "+" + model.length - 6
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Rectangle {
            anchors {
                right: parent.right
                left: parent.left
                top: tableView.bottom
            }
            height: 1
            color: CmnCfg.palette.black
        }
    }
}
