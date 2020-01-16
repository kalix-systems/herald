import QtQuick 2.14
import QtQuick.Window 2.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "qrc:/imports/errors"
import "ChatView/Popups" as CvPopups

ApplicationWindow {
    id: root
    title: qsTr("Herald")
    visible: true
    width: 900
    height: 640
    minimumWidth: 500
    minimumHeight: 300

    ErrorDialog {
        id: errPopup
    }

    property alias ccMap: ccMap.map
    Item {
        // conversation content map
        id: ccMap

        property var map: ({})
    }

    Connections {
        target: Herald.errors
        onNewError: {
            const errMsg = Herald.errors.nextError()

            if (errMsg !== "") {
                errPopup.errorMsg = errMsg
                errPopup.open()
            }
        }
    }

    EmojiPicker {
        id: emojiPickerModel
    }

    Loader {
        id: galleryLoader
        anchors.fill: active ? parent : undefined
        property var imageAttachments
        property int currentIndex: 0
        active: false
        sourceComponent: CvPopups.GalleryView {
            id: galleryView
        }
    }

    Loader {
        id: appLoader
        active: Herald.configInit
        anchors.fill: parent
        sourceComponent: App {}
    }

    Loader {
        id: registrationLoader
        anchors.fill: parent
        active: !Herald.configInit
        sourceComponent: RegistrationPage {}
    }
}
