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
            height: CmnCfg.toolbarHeight + 1
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

        ListView {
            id: tableView
            boundsBehavior: Flickable.StopAtBounds
            boundsMovement: Flickable.StopAtBounds
            anchors.fill: parent
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
                        Layout.maximumWidth: size
                        Layout.margins: CmnCfg.defaultMargin
                        height: CmnCfg.avatarSize - CmnCfg.defaultMargin
                        pfpPath: profilePicture
                        color: CmnCfg.avatarColors[model.color]
                        initials: Utils.initialize(name)
                    }

                    Column {
                        Layout.alignment: Qt.AlignLeft
                        Layout.maximumWidth: 85
                        Label {
                            text: userId
                        }
                        Label {
                            text: "@" + name
                        }
                    }

                    IconButton {
                        Layout.minimumWidth: CmnCfg.largeMargin
                    }

                    Flow {
                        Layout.preferredWidth: 85
                        Layout.preferredHeight: 45
                        spacing: CmnCfg.microMargin
                        topPadding: 3
                        Repeater {
                            model: [0, 1, 2, 3, 4, 5, 6, 7, 8].slice(0, 6)
                            Rectangle {
                                id: imagePorxy
                                color: "green"
                                width: 20
                                height: 20
                            }
                        }
                    }

                    Label {
                        Layout.preferredWidth: 140
                        text: "Location Location Location Location Location Location"
                        wrapMode: Label.WrapAtWordBoundaryOrAnywhere
                    }

                    Flow {
                        Layout.preferredWidth: 85
                        Repeater {
                            model: 3
                            Label {
                                background: Rectangle {
                                    width: index * 33
                                    height: 10
                                    color: "blue"
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
