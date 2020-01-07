import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.13
import QtQuick.Window 2.2
import LibHerald 1.0
import "qrc:/imports"

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
                    // top padding aligns headerText baseline with baseline of
                    // initial in user avatar to right
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
                height: 10
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
