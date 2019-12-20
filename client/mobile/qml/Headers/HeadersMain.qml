import QtQuick 2.14
import QtQuick.Controls 2.14

Rectangle {
    id: headerRoot
    // header is initially empty, flat and colorless
    Loader {
        id: rootLoader
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
}
