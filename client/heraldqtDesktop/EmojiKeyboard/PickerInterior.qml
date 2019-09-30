import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0

Item {
    property color lowlight: "light gray"
    readonly property int categoryCount: 8
    // header and search bar
    Item {
        id: header
        height: 30
        anchors.top: parent.top
        anchors.topMargin: 10
        anchors.right: parent.right
        anchors.left: parent.left

        // search bar and exit button
        Rectangle {
            id: taBox
            ScrollView {
                anchors {
                    left: parent.left
                    right: exitButton.left
                    leftMargin: 0
                }
                TextArea {
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
                anchors.top: parent.top
                anchors.bottom: parent.bottom
                anchors.right: parent.right
                anchors.margins: QmlCfg.smallMargin - 1
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
            radius: QmlCfg.radius
            border.color: "white"
            border.width: 0.5
            height: 25
        }

        // skin swatch selector
        ComboBox {
            id: menu
            anchors.right: parent.right
            anchors.margins: QmlCfg.margin
            anchors.verticalCenter: taBox.verticalCenter
            height: 20
            width: 20
            currentIndex: QmlCfg.skinSwatchIndex
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
                QmlCfg.skinSwatchIndex = currentIndex
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
            anchors.fill: parent
            boundsBehavior: Flickable.StopAtBounds
            clip: true
            contentHeight: col.height
            ScrollBar.vertical: ScrollBar {}
            Column {
                id: col
                spacing: QmlCfg.margin
                Repeater {
                    id: categoryBlock
                    model: categoryCount
                    Column {
                        spacing: QmlCfg.smallMargin
                        topPadding: QmlCfg.margin


                        Text {
                            leftPadding: QmlCfg.smallMargin
                            text: "Category Name"
                            font.bold: true
                        }
                        Grid {
                            id: emojiGrid
                            columns: 8
                            spacing: 2
                            Repeater {
                                model: 101
                                EmojiButton {}
                            }
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
            height: 0.5
            color: "white"
        }

        Row {
            anchors {
                topMargin: QmlCfg.margin
                top: hr.bottom
                horizontalCenter: hr.horizontalCenter
            }
            spacing: QmlCfg.margin
            AnchorButton {
                lowlight: lowlight
                imageSource: "qrc:/emoji-categories/gestural.svg"
            }
            AnchorButton {
                lowlight: lowlight
                imageSource: "qrc:/emoji-categories/nature.svg"
            }
            AnchorButton {
                lowlight: lowlight
                imageSource: "qrc:/emoji-categories/food.svg"
            }
            AnchorButton {
                lowlight: lowlight
                imageSource: "qrc:/emoji-categories/sports.svg"
            }
            AnchorButton {
                lowlight: lowlight
                imageSource: "qrc:/emoji-categories/transport.svg"
            }
            AnchorButton {
                lowlight: lowlight
                imageSource: "qrc:/emoji-categories/items.svg"
            }
            AnchorButton {
                lowlight: lowlight
                imageSource: "qrc:/emoji-categories/symbols.svg"
            }
            AnchorButton {
                lowlight: lowlight
                imageSource: "qrc:/emoji-categories/flags.svg"
            }
        }
    }
}
