import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.12

/// --- displays a list of contacts
Row {
    property string displayName: ""
    property string pfpUrl: ""
    property int colorHash: 0
    property int shapeEnum: 0 /// { individual, group ... }
    readonly property var set_new_image: init
    spacing: 10

    ///--- Circle with initial
    leftPadding: 10
    anchors.verticalCenter: parent.verticalCenter

    Component.onCompleted: {
        init()
    }

    Item {
        width: rowHeight - 10
        height: rowHeight - 10
        id: dummy
    }

    Text {
        text: displayName
        font.bold: true
        anchors.verticalCenter: parent.verticalCenter
    }

    ///--- potential avatar components
    Component {
        id: initialAvatar
        Rectangle {
            width: rowHeight - 10
            height: rowHeight - 10
            anchors.verticalCenter: parent.verticalCenter
            color: QmlCfg.avatarColors[colorHash]
            radius: shapeEnum == 0 ? width : 0
            ///---- initial
            Text {
                text: qsTr(displayName[0].toUpperCase())
                font.bold: true
                color: "white"
                anchors.centerIn: parent
                font.pixelSize: parent.height - 5
            }
        }
    }

    Component {
        id: imageAvatar
        Image {
            width: rowHeight - 10
            height: rowHeight - 10
            source: "file:" + pfpUrl
            asynchronous: true
            mipmap: true
        }
    }

    function init() {
        print("call of init")
        if (pfpUrl === "")
            replaceElement(initialAvatar)
        else
            replaceElement(imageAvatar)
    }

    function replaceElement(newElementFactory) {
        print("call of replace")

        var oldChild = dummy.childAt(0, 0)
        if (oldChild !== null)
            oldChild.destroy()

        var element = newElementFactory.createObject(dummy, {

                                                     })
        if (element === null)
            print("Error creating object")
    }
}
