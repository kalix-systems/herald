import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0

Item {
    property color lowlight: "light gray"
    // header and search bar
    Item {
        id: header
        height: 30 //enough for search bar of default size w/ margins
        anchors.top: parent.top
        anchors.topMargin: CmnCfg.margin
        anchors.right: parent.right
        anchors.left: parent.left

        // search bar and exit button
        Rectangle {
            id: taBox
            ScrollView {
                anchors {
                    left: parent.left
                    right: exitButton.left
                }
                TextArea {
                    id: searchTextArea
                    placeholderText: "Search..."
                    Keys.onReturnPressed: {
                        event.accepted = true
                    }
                }
            }
            Button {
                id: exitButton
                background: Rectangle {
                    color: parent.pressed ? "#33000000" : "#44000000" // transparent
                    radius: parent.height
                    anchors.fill: parent
                }
                anchors {
                    top: parent.top
                    bottom: parent.bottom
                    right: parent.right
                    margins: CmnCfg.margin - 5
                }
                width: height
                onClicked: emoKeysPopup.active = false
                Text {
                    text: "X"
                    anchors.centerIn: parent
                }
            }

            anchors {
                left: parent.left
                right: menu.left
                margins: 10
            }

            color: "#33000000" // transparent
            radius: CmnCfg.radius
            border.color: "white"
            border.width: 0.5
            height: 25
        }

        // skin swatch selector
        ComboBox {
            id: menu
            anchors.right: parent.right
            anchors.margins: CmnCfg.margin
            anchors.verticalCenter: taBox.verticalCenter
            height: 20
            width: 20
            currentIndex: CmnCfg.skinSwatchIndex
            model: ["#f4be40", "#f9dcbe", "#dfbb97", "#c18f6b", "#9a6440", "#59453a"]
            indicator: Item {}
            delegate: ItemDelegate {
                height: menu.height
                Rectangle {
                    anchors.fill: parent
                    color: menu.model[index]
                }
            }

            onCurrentIndexChanged: {
                CmnCfg.skinSwatchIndex = currentIndex
            }

            contentItem: Rectangle {
                anchors.fill: parent
                color: menu.model[menu.currentIndex]
            }
        }
    }

    Rectangle {
        width: parent.width
        height: 0.5
        color: "white"
        anchors.bottom: listView.top
    }

    // actual interior
    Item {
        id: listView
        width: parent.width
        anchors {
            top: header.bottom
            bottom: footer.top
        }
        Flickable {
            id: emojiList
            anchors.fill: parent
            boundsBehavior: Flickable.StopAtBounds
            clip: true
            ScrollBar.vertical: ScrollBar {}
            contentHeight: innerCol.height
            Column {
                id: innerCol
                Repeater {
                    id: innerRepeater
                    model: searchTextArea.text.length ? [] : CmnCfg.emojiModel
                    Column {
                        Text {
                            padding: CmnCfg.smallMargin
                            text: modelData.sectionName
                            font.bold: true
                        }
                        Component {
                            id: emojiComp
                            Grid {
                                id: emojiGrid
                                columns: 8
                                spacing: 2
                                Repeater {
                                    id: self
                                    model: modelData.List
                                    EmojiButton {
                                        baseEmoji: self.model[index][0]
                                        takesModifier: self.model[index].length === 3
                                    }
                                }
                            }
                        }
                        Loader {
                            sourceComponent: emojiComp
                            asynchronous: index > 0
                        }
                    }
                }
            }
        }
    }

    // footer and anchor links
    Item {
        id: footer
        width: parent.width
        height: 30
        anchors.bottom: parent.bottom
        anchors.bottomMargin: 20 // 10 + carat height

        Rectangle {
            id: hr
            width: parent.width
            height: 1
            color: "white"
        }

        Row {
            anchors {
                topMargin: CmnCfg.margin
                top: hr.bottom
                horizontalCenter: hr.horizontalCenter
            }
            spacing: CmnCfg.smallMargin
            AnchorButton {
                lowlight: lowlight
                anchorIndex: 0
                imageSource: "qrc:/emoji-categories/gestural.svg"
            }
            AnchorButton {
                lowlight: lowlight
                anchorIndex: 1
                imageSource: "qrc:/emoji-categories/nature.svg"
            }
            AnchorButton {
                lowlight: lowlight
                anchorIndex: 2
                imageSource: "qrc:/emoji-categories/food.svg"
            }
            AnchorButton {
                lowlight: lowlight
                anchorIndex: 3
                imageSource: "qrc:/emoji-categories/transport.svg"
            }
            AnchorButton {
                lowlight: lowlight
                anchorIndex: 4
                imageSource: "qrc:/emoji-categories/sports.svg"
            }
            AnchorButton {
                lowlight: lowlight
                anchorIndex: 5
                imageSource: "qrc:/emoji-categories/items.svg"
            }
            AnchorButton {
                lowlight: lowlight
                anchorIndex: 6
                imageSource: "qrc:/emoji-categories/symbols.svg"
            }
            AnchorButton {
                lowlight: lowlight
                anchorIndex: 7
                imageSource: "qrc:/emoji-categories/flags.svg"
            }
        }
    }
}
