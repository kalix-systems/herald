import QtQuick 2.14
import QtQuick.Controls 2.14

ToolBar {
    id: headerRoot
    height: CmnCfg.toolbarHeight
    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }

    LoginHeader {
        id: loginHeader
    }

    // header is initially empty, flat and colorless
    Loader {
        id: rootLoader
        anchors.fill: parent
        sourceComponent: loginHeader
    }

    states: [
        State {
            // no header
            name: "login"
            when: loginPageLoader.active
            PropertyChanges {}
        },
        State {
            name: "home"
            when: appLoader.sourceComponent.id === "cvMain"
            PropertyChanges {}
        },
        State {
            name: "chat"
            PropertyChanges {}
        },
        State {
            name: "contacts"
            PropertyChanges {}
        },
        State {
            name: "config"
            PropertyChanges {}
        }
    ]

    Transitions {
        id: trs
    }

    transitions: trs.transitionsArray
}
