import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Window 2.13
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import "qrc:/imports" as Imports
import QtGraphicalEffects 1.0
import "../../common" as Common
import "qrc:/imports/Entity" as Entity
import "qrc:/imports/js/utils.mjs" as Utils
import QtQuick.Layouts 1.3

import QtQuick.Shapes 1.12

Popup {
    id: groupSettingsPopup
    property var convoData: parent.convoData
    property Members convoMembers: parent.convoMembers

    padding: 0
    height: chatView.height
    width: parent.width
    anchors.centerIn: parent
    onClosed: groupSettingsLoader.active = false
    modal: true

    background: Rectangle {
        id: background
        color: CmnCfg.palette.white
    }

    Imports.IconButton {
        anchors.right: parent.right
        anchors.rightMargin: CmnCfg.defaultMargin
        anchors.verticalCenter: header.verticalCenter
        icon.source: "qrc:/x-icon.svg"
        fill: CmnCfg.palette.white
        onClicked: {
            groupSettingsPopup.close()
            groupSettingsLoader.active = false
        }
        z: header.z + 1
    }

    Rectangle {
        id: header
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: CmnCfg.toolbarHeight + 1
        color: CmnCfg.palette.offBlack
        Label {
            id: headerLabel
            anchors.left: parent.left
            anchors.leftMargin: CmnCfg.smallMargin
            text: qsTr("Group settings")
            font.pixelSize: CmnCfg.headerFontSize
            color: CmnCfg.palette.white
            anchors.verticalCenter: parent.verticalCenter
            font.family: CmnCfg.labelFont.name
        }
    }

    Flickable {
        width: parent.width
        anchors.top: header.bottom
        anchors.bottom: parent.bottom
        contentWidth: width
        contentHeight: wrapperCol.height
        clip: true
        ScrollBar.vertical: ScrollBar {}
        boundsBehavior: Flickable.StopAtBounds
        Column {
            id: wrapperCol
            width: parent.width - CmnCfg.smallMargin * 2
            anchors.horizontalCenter: parent.horizontalCenter
            spacing: CmnCfg.smallMargin
            padding: CmnCfg.smallMargin

            RowLayout {
                Rectangle {
                    height: 60
                    width: height

                    Layout.alignment: Qt.AlignVCenter

                    Entity.Avatar {
                        id: avatar
                        pfpPath: Utils.safeStringOrDefault(convoData.picture,
                                                           "")

                        color: CmnCfg.palette.avatarColors[convoData.conversationColor]
                        size: parent.height
                        textColor: CmnCfg.palette.iconFill
                        initials: Utils.initialize(convoData.title)
                        isGroup: !convoData.pairwise

                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: picturePopup.open()
                        }
                    }

                    Shape {
                        id: cornerCarat
                        anchors {
                            right: avatar.right
                            bottom: avatar.bottom
                        }
                        anchors.fill: parent
                        ShapePath {
                            fillColor: CmnCfg.palette.darkGrey
                            strokeColor: "#00000000"
                            startX: cornerCarat.width * .8
                            startY: cornerCarat.height
                            PathLine {
                                x: cornerCarat.width
                                y: cornerCarat.height * .8
                            }
                            PathLine {
                                x: cornerCarat.width
                                y: cornerCarat.height
                            }
                            PathLine {
                                x: cornerCarat.width * .8
                                y: cornerCarat.height
                            }
                        }
                    }
                }

                Imports.BorderedTextField {
                    id: title
                    text: convoData.title
                    selectByMouse: true
                    selectionColor: CmnCfg.palette.highlightColor
                    readOnly: true
                    font.family: CmnCfg.chatFont.name
                    font.pixelSize: CmnCfg.labelFontSize
                    font.weight: Font.Medium
                    color: CmnCfg.palette.black
                    borderColor: CmnCfg.palette.white

                    Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                    Layout.leftMargin: CmnCfg.megaMargin
                    Layout.preferredWidth: title.contentWidth
                }

                Imports.IconButton {
                    id: titleEditButton
                    fill: CmnCfg.palette.black
                    source: "qrc:/pencil-icon.svg"
                    Layout.alignment: Qt.AlignLeft
                    Layout.leftMargin: CmnCfg.microMargin
                    property bool editing: false

                    onClicked: {
                        if (editing) {
                            titleEditButton.editing = false
                            title.readOnly = true
                            title.borderColor = CmnCfg.palette.white
                            title.Layout.fillWidth = false
                            titleEditButton.source = "qrc:/pencil-icon.svg"
                            convoData.title = title.text
                        } else {
                            titleEditButton.editing = true
                            title.readOnly = false
                            title.borderColor = CmnCfg.palette.black
                            title.Layout.fillWidth = true
                            titleEditButton.source = "qrc:/check-icon.svg"
                        }
                    }
                }
            }
            Row {
                height: memberHeader.height
                spacing: CmnCfg.defaultMargin
                Text {
                    text: "Members"
                    id: memberHeader
                    font.family: CmnCfg.chatFont.name
                    font.weight: Font.Medium
                    font.pixelSize: CmnCfg.defaultFontSize
                }

                Imports.IconButton {
                    id: memberExpand
                    icon.source: memberList.visible ? "qrc:/up-chevron-icon" : "qrc:/down-chevron-icon"
                    fill: CmnCfg.palette.black
                    anchors.verticalCenter: memberHeader.verticalCenter
                    onClicked: memberList.visible = !memberList.visible
                }
            }

            ListView {
                id: memberList
                height: contentHeight
                width: parent.width
                model: convoMembers
                interactive: false
                highlightFollowsCurrentItem: false
                currentIndex: -1
                delegate: Item {
                    height: visible ? CmnCfg.convoHeight : 0
                    width: parent.width
                    property var memberData: UserMap.get(model.userId)
                    Common.PlatonicRectangle {
                        boxTitle: memberData.name
                        boxColor: memberData.userColor
                        picture: Utils.safeStringOrDefault(
                                     memberData.profilePicture, "")
                        color: CmnCfg.palette.white
                        topTextMargin: CmnCfg.smallMargin
                        bottomTextMargin: CmnCfg.defaultMargin
                        labelComponent: Entity.ContactLabel {
                            displayName: memberData.name
                            username: memberData.userId
                        }
                        states: []
                        MouseArea {
                            id: hoverHandler
                        }
                    }
                }
            }
        }
    }
    FileDialog {
        id: picturePopup
        property bool pfpValid: true
        folder: shortcuts.desktop
        nameFilters: ["(*.jpg *.png *.jpeg)"]
        onSelectionAccepted: {
            var parsed = JSON.parse(Herald.utils.imageDimensions(fileUrl))

            const picture = {
                "width": Math.round(parsed.width),
                "height": Math.round(parsed.height),
                "x": 0,
                "y": 0,
                "path": Herald.utils.stripUrlPrefix(fileUrl)
            }

            Herald.conversations.setProfilePicture(
                        Herald.conversations.indexById(
                            convoData.conversationId), JSON.stringify(picture))
            //commented out until image crop popup is fixed
            //            imageCrop.imageWidth = parsed.width
            //            imageCrop.imageHeight = parsed.height
            //            imageCrop.imageSource = fileUrl
            //            imageCrop.show()
        }
    }
}
