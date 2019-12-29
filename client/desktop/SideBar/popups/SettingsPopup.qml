import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import QtQuick.Layouts 1.13
import QtQuick.Window 2.2
import LibHerald 1.0
import "qrc:/imports"
import "../../common" as Common
import "./SettingsComponents" as SetsComps
import "./js/SettingsPopupSubmission.mjs" as JS

Window {
    id: settingsPopup
    width: CmnCfg.settingsPaneWidth
    height: CmnCfg.settingsPaneHeight
    minimumWidth: 500
    minimumHeight: 250

    Component.onCompleted: {
        x = root.x + root.width / 3
        y = root.y + 100
    }

    FileDialog {
        id: settingsProfPic
        property bool pfpValid: true
        folder: shortcuts.desktop
        nameFilters: ["(*.jpg *.png *.jpeg)"]
        onSelectionAccepted: Herald.config.profilePicture = fileUrl
    }

    Page {
        anchors.fill: parent
        header: Rectangle {
            id: headerRect
            color: CmnCfg.palette.offBlack
            height: CmnCfg.toolbarHeight

            Label {
                id: label
                text: qsTr("General settings")
                color: CmnCfg.palette.white
                font.pixelSize: CmnCfg.headerFontSize
                font.family: CmnCfg.labelFont.name
                font.weight: Font.DemiBold
                anchors.verticalCenter: parent.verticalCenter
                elide: Label.ElideRight
                leftPadding: CmnCfg.defaultMargin
            }

            Rectangle {
                height: 1
                width: parent.width
                color: CmnCfg.palette.white
                anchors.top: headerRect.bottom
            }
        }

        ListModel {
            id: settingsModel
            ListElement {
                name: qsTr("Profile information")
            }
            ListElement {
                name: qsTr("Notifications")
            }
            ListElement {
                name: qsTr("Appearance")
            }
            ListElement {
                name: qsTr("Privacy & Security")
            }
            ListElement {
                name: qsTr("Data & Storage")
            }

            ListElement {
                name: qsTr("Advanced")
            }

            ListElement {
                name: qsTr("Help & Feedback")
            }
        }

        RowLayout {
            anchors.fill: parent
            spacing: 0

            Rectangle {
                id: headersRect
                Layout.preferredWidth: 250
                Layout.fillHeight: true
                color: CmnCfg.palette.offBlack

                ListView {
                    anchors.fill: parent
                    // align with first header in right pane ListView
                    anchors.topMargin: 2
                    model: settingsModel
                    delegate: Rectangle {
                        height: 40
                        width: parent.width
                        color: hover.containsMouse ? CmnCfg.palette.darkGrey : "transparent"
                        StandardLabel {
                            text: name
                            font.family: CmnCfg.labelFont.name
                            font.weight: Font.DemiBold
                            anchors.verticalCenter: parent.verticalCenter
                            anchors.left: parent.left
                            anchors.leftMargin: CmnCfg.defaultMargin
                        }
                        MouseArea {
                            id: hover
                            hoverEnabled: true
                            anchors.fill: parent
                            onClicked: settingsScroll.contentY = col.children[index].y
                            cursorShape: Qt.PointingHandCursor
                        }
                    }
                }
            }

            Flickable {
                id: settingsScroll
                Layout.fillWidth: true
                Layout.fillHeight: true
                contentHeight: col.height
                boundsBehavior: Flickable.StopAtBounds

                Column {
                    id: col
                    spacing: CmnCfg.smallMargin
                    topPadding: CmnCfg.smallMargin
                    anchors.right: parent.right
                    anchors.left: parent.left
                    SetsComps.SettingsListItem {
                        id: notifications
                        headerText: qsTr("Profile information")
                        settingsContent: SetsComps.Profile {}
                    }
                    SetsComps.SettingsListItem {
                        id: profile
                        headerText: qsTr("Notifications")
                        settingsContent: SetsComps.Notifications {}
                    }
                    SetsComps.SettingsListItem {
                        id: appearance
                        headerText: qsTr("Appearance")
                        settingsContent: SetsComps.Appearance {}
                    }
                    SetsComps.SettingsListItem {
                        id: security
                        headerText: qsTr("Privacy & Security")
                        settingsContent: SetsComps.Privacy {}
                    }

                    SetsComps.SettingsListItem {
                        id: storage
                        headerText: qsTr("Data & Storage")
                        settingsContent: SetsComps.Storage {}
                    }

                    SetsComps.SettingsListItem {
                        id: advanced
                        headerText: qsTr("Advanced")
                        settingsContent: SetsComps.Advanced {}
                    }

                    SetsComps.SettingsListItem {
                        id: feedback
                        headerText: qsTr("Help & Feedback")
                        settingsContent: SetsComps.Feedback {}
                    }
                }
            }
        }
    }
}
