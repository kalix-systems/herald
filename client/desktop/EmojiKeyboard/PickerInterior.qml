import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.12
import QtQuick.Shapes 1.13
import LibHerald 1.0

Item {
    property color lowlight: "light gray"

    // header and search bar
    ListModel {
        id: anchorModel
        ListElement {
            sectionName: "Recents"
            imageSource: "qrc:/emoji-categories/recents.svg"
        }

        ListElement {
            sectionName: "Smileys & Emotion"
            imageSource: "qrc:/emoji-categories/gestural.svg"
        }
        ListElement {
            sectionName: "Animals & Nature"
            imageSource: "qrc:/emoji-categories/nature.svg"
        }
        ListElement {
            sectionName: "Food & Drink"
            imageSource: "qrc:/emoji-categories/food.svg"
        }
        ListElement {
            sectionName: "Travel & Places"
            imageSource: "qrc:/emoji-categories/transport.svg"
        }
        ListElement {
            sectionName: "Activities"
            imageSource: "qrc:/emoji-categories/sports.svg"
        }
        ListElement {
            sectionName: "Objects"
            imageSource: "qrc:/emoji-categories/items.svg"
        }
        ListElement {
            sectionName: "Symbols"
            imageSource: "qrc:/emoji-categories/symbols.svg"
        }
        ListElement {
            sectionName: "Flags"
            imageSource: "qrc:/emoji-categories/flags.svg"
        }
    }

    Item {
        id: header
        height: 30 //enough for search bar of default size w/ margins
        anchors.top: parent.top
        anchors.topMargin: CmnCfg.smallMargin
        anchors.right: parent.right
        anchors.left: parent.left

        // search bar and exit button
        Rectangle {
            id: taBox
            anchors {
                left: parent.left
                right: menu.left
                margins: CmnCfg.smallMargin
            }
            color: "#33000000" // transparent
            border.color: CmnCfg.palette.medGrey
            height: 24
            Row {
                anchors.fill: parent
                spacing: 0
                Button {
                    padding: 0
                    background: Item {}
                    icon.source: "qrc:/search-icon.svg"
                    icon.color: CmnCfg.palette.medGrey
                    icon.height: 17
                    icon.width: 17
                    anchors.verticalCenter: parent.verticalCenter
                }

                TextArea {
                    id: searchTextArea
                    padding: 0
                    topPadding: 3
                    color: CmnCfg.palette.medGrey
                    anchors.verticalCenter: parent.verticalCenter
                    placeholderText: "Search emoji"
                    Keys.onReturnPressed: event.accepted = true
                    width: 185
                    height: parent.height
                    onTextChanged: emojiPickerModel.setSearchString(text)
                }

                Button {
                    id: exitButton
                    padding: 0
                    background: Item {}
                    icon.source: "qrc:/x-icon.svg"
                    icon.color: CmnCfg.palette.medGrey
                    icon.height: 17
                    icon.width: 17
                    onClicked: emoKeysPopup.active = false
                    anchors.verticalCenter: parent.verticalCenter
                }
            }
        }

        // skin swatch selector
        ComboBox {
            id: menu
            anchors.right: parent.right
            anchors.margins: CmnCfg.defaultMargin
            anchors.verticalCenter: taBox.verticalCenter
            height: 24
            width: 24
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
            onCurrentIndexChanged: CmnCfg.skinSwatchIndex = currentIndex
            contentItem: Rectangle {
                anchors.fill: parent
                border.color: CmnCfg.palette.darkGrey
                color: menu.model[menu.currentIndex]
                Shape {
                    id: cornerCarat
                    anchors {
                        right: parent.right
                        bottom: parent.bottom
                    }
                    anchors.fill: parent
                    ShapePath {
                        fillColor: CmnCfg.palette.darkGrey
                        strokeColor: "#00000000"
                        startX: cornerCarat.width / 2
                        startY: cornerCarat.height
                        PathLine {
                            x: cornerCarat.width
                            y: cornerCarat.height / 2
                        }
                        PathLine {
                            x: cornerCarat.width
                            y: cornerCarat.height
                        }
                        PathLine {
                            x: cornerCarat.width / 2
                            y: cornerCarat.height
                        }
                    }
                }
            }
        }
    }

    // actual interior
    Item {
        id: listView

        anchors {
            top: header.bottom
            bottom: footer.top
            right: parent.right
            left: parent.left
            margins: CmnCfg.smallMargin
        }
        // z is higher than mouse area capturing events behind parent to prevent chat bubble flicker
        z: parent.z + 1

        Loader {
            id: listLoader
            asynchronous: true
            anchors.fill: parent
            signal position(int index)
            sourceComponent: StandardInterior {
                id: emojiList
                Connections {
                    target: parent
                    onPosition: {
                        emojiList.positionViewAtIndex(index, 0)
                    }
                }
            }
            Component {
                id: searchComp
                SearchInteriorComponent {}
            }

            states: [
                State {
                    name: "default"
                    when: searchTextArea.text.length !== 0
                    PropertyChanges {
                        target: listLoader
                        sourceComponent: searchComp
                    }
                }
            ]
        }
    }
    // footer and anchor links
    Item {
        id: footer

        // z is higher than mouse area capturing events behind parent to prevent chat bubble flicker
        z: parent.z + 1
        anchors.bottom: parent.bottom
        width: parent.width
        height: 30

        Rectangle {
            id: hr
            width: parent.width - 2
            height: 1
            anchors.horizontalCenter: parent.horizontalCenter
            color: CmnCfg.palette.darkGrey
        }

        RowLayout {
            id: anchorRow
            anchors.fill: parent
            anchors.margins: 8
            spacing: CmnCfg.smallMargin
            Repeater {
                model: anchorModel
                AnchorButton {
                    sectionName: model.sectionName
                    imageSource: model.imageSource
                }
            }
        }
    }
}
