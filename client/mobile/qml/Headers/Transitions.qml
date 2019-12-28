import QtQuick 2.14

// a container item for transitions used
// in the header
Item {
    property var transitionsArray: [tloginHome]

    Transition {
        id: tloginHome
        from: "login"
        to: "home"
    }
}
