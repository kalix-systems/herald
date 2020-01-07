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
    minimumWidth: 700
    minimumHeight: 450

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
                    property bool searchRegex: false
                    fill: CmnCfg.palette.lightGrey
                    source: "qrc:/search-icon.svg"
                }

                Item {
                    Layout.preferredWidth: CmnCfg.defaultMargin
                }

                IconButton {
                    id: settignsButton
                    property bool searchRegex: false
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
                Layout.alignment: Qt.AlignLeft
                Layout.preferredWidth: 32
                height: 1
            }
            Text {
                font.bold: true
                Layout.alignment: Qt.AlignLeft
                Layout.preferredWidth: CmnCfg.avatarSize
                text: "Name"
            }
            Text {
                font.bold: true
                Layout.alignment: Qt.AlignLeft
                Layout.minimumWidth: CmnCfg.largeMargin
                text: "Trusted"
            }
            Text {
                font.bold: true
                Layout.alignment: Qt.AlignHCenter
                Layout.preferredWidth: 85
                text: "Location"
            }
            Text {
                font.bold: true
                Layout.alignment: Qt.AlignRight
                Layout.preferredWidth: 85
                text: "Tags"
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
                        Layout.margins: CmnCfg.defaultMargin
                        height: CmnCfg.avatarSize - CmnCfg.defaultMargin
                        pfpPath: profilePicture
                        color: CmnCfg.avatarColors[model.color]
                        initials: Utils.initialize(name)
                    }

                    Column {
                        Layout.alignment: Qt.AlignLeft
                        Layout.preferredWidth: 85
                        spacing: CmnCfg.smallMargin
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
                    }

                    Flow {
                        Layout.alignment: Qt.AlignLeft
                        Layout.preferredWidth: 85
                        Layout.preferredHeight: 45
                        spacing: CmnCfg.microMargin
                        topPadding: 3
                        Repeater {
                            model: 0
                            Rectangle {
                                id: imagePorxy
                                color: "green"
                                width: 20
                                height: 20
                            }
                        }
                    }

                    Label {
                        Layout.alignment: Qt.AlignLeft
                        Layout.preferredWidth: 140
                        text: "Location Location Location Location Location Location"
                        wrapMode: Label.WrapAtWordBoundaryOrAnywhere
                    }

                    Flow {
                        Layout.preferredWidth: 85
                        Repeater {
                            model: 0
                            Label {
                                background: Rectangle {}
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
