import QtQuick 2.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "."

ToolBar {
    id: headerRoot

    property StackView mainStackView
    // global title used dynamically by multiple headers
    property string title

    height: CmnCfg.toolbarHeight

    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }

    // header is initially empty, flat and colorless
    Loader {
        id: rootLoader
        anchors.fill: parent
    }

    Component {
        id: homeHeader
        HomeHeader {}
    }

    Component {
        id: chatHeader
        ChatHeader {}
    }

    Component {
        id: confHeader
        ConfigHeader {}
    }

    Component {
        id: newContactHeader
        NewContactHeader {}
    }

    Component {
        id: newGroupHeader
        NewGroupHeader {}
    }

    states: [
        State {
            name: "home"
            PropertyChanges {
                target: rootLoader
                sourceComponent: homeHeader
            }
        },
        State {
            name: "chat"
            PropertyChanges {
                target: rootLoader
                sourceComponent: chatHeader
            }
            PropertyChanges {
                target: headerRoot
                title: mainView.currentItem.headerTitle
            }
        },
        State {
            name: "contacts"
            PropertyChanges {}
        },
        State {
            name: "newGroup"
            PropertyChanges {
                target: rootLoader
                sourceComponent: newGroupHeader
            }
        },
        State {
            name: "newContact"
            PropertyChanges {
                target: rootLoader
                sourceComponent: newContactHeader
            }
        },
        State {
            name: "config"
            PropertyChanges {
                target: rootLoader
                sourceComponent: confHeader
            }
        }
    ]

    Transitions {
        id: trs
    }

    transitions: trs.transitionsArray
}
