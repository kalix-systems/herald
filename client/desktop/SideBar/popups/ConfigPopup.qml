import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.13
import QtQuick.Window 2.2
import LibHerald 1.0
import "qrc:/imports"
import "../../common" as Common
import "./ConfigComponents" as CfgComps
import "./js/ConfigPopupSubmission.mjs" as JS

Window {
    id: configPopup
    width: CmnCfg.configWidth
    height: CmnCfg.configHeight

    Component.onCompleted: {
        x = root.x + root.width / 3
        y = root.y + 100
    }

    FileDialog {
        id: cfgPfp
        property bool pfpValid: true
        folder: shortcuts.desktop
        nameFilters: ["(*.jpg *.png *.jpeg)"]
        onSelectionAccepted: {
            herald.config.profilePicture = fileUrl
            print("set to", fileUrl)
        }
    }

    Page {
        anchors.fill: parent

        header: Rectangle {
            id: headerRect
            color: CmnCfg.palette.offBlack
            height: CmnCfg.toolbarHeight
            Row {
                leftPadding: CmnCfg.margin
                anchors.fill: parent
                Label {
                    id: label
                    text: qsTr("Settings")
                    color: CmnCfg.palette.white
                    font.pixelSize: CmnCfg.headerSize
                    font.family: CmnCfg.labelFont.name
                    font.bold: true
                    anchors.verticalCenter: parent.verticalCenter
                    elide: Label.ElideRight
                }
            }
            Rectangle {
                height: 1
                width: parent.width
                color: CmnCfg.palette.white
                anchors.top: headerRect.bottom
            }
        }

        Row {
            anchors.fill: parent
            Rectangle {
                width: 0.3 * 600
                height: parent.height
                color: CmnCfg.palette.offBlack
                Column {
                    spacing: CmnCfg.margin
                    padding: CmnCfg.margin
                    StandardLabel {
                        text: qsTr("Notifications")
                    }
                    StandardLabel {
                        text: qsTr("Appearance")
                    }
                    StandardLabel {
                        text: qsTr("Privacy & Security")
                    }
                    StandardLabel {
                        text: qsTr("Data & Storage")
                    }
                    StandardLabel {
                        text: qsTr("Advanced")
                    }
                    StandardLabel {
                        text: qsTr("Help & Feedback")
                    }
                }
            }

            Column {
                spacing: CmnCfg.margin
                padding: CmnCfg.margin
                CfgComps.ConfigListItem {
                    headerText: qsTr("Notifications")
                    configContent: CfgComps.Notifications {}
                }
                CfgComps.ConfigListItem {
                    headerText: qsTr("Appearance")
                }
                CfgComps.ConfigListItem {
                    headerText: "Privacy & Security"
                }

                CfgComps.ConfigListItem {
                    headerText: "Data & Storage"
                }

                CfgComps.ConfigListItem {
                    headerText: "Advanced"
                }

                CfgComps.ConfigListItem {
                    headerText: "Help & Feedback"
                }
            }
        }
    }
}
