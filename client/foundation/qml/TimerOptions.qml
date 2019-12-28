import QtQuick 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQml 2.13
import QtQuick.Controls 2.5
import Qt.labs.platform 1.0

Menu {
    id: optMenu

    // Item in `Conversations` model
    property var conversationItem
    property int chosenPeriod: conversationItem.expirationPeriod
    property string chosenTimer: timerModel.get(chosenPeriod).path

    // TODO real icon
    ListModel {
        id: timerModel

        ListElement {
            name: qsTr("Off")
            path: "qrc:/timer-option-icons/off.svg"
        }

        ListElement {
            name: qsTr("30 seconds")
            path: "qrc:/timer-option-icons/30s.svg"
        }

        ListElement {
            name: qsTr("1 minute")
            path: "qrc:/timer-option-icons/1min.svg"
        }

        ListElement {
            name: qsTr("30 minutes")
            path: "qrc:/timer-option-icons/30min.svg"
        }

        ListElement {
            name: qsTr("1 hour")
            path: "qrc:/timer-option-icons/1h.svg"
        }

        ListElement {
            name: qsTr("12 hours")
            path: "qrc:/timer-option-icons/12h.svg"
        }

        ListElement {
            name: qsTr("1 day")
            path: "qrc:/timer-option-icons/1d.svg"
        }

        ListElement {
            name: qsTr("1 week")
            path: "qrc:/timer-option-icons/1w.svg"
        }

        ListElement {
            name: qsTr("1 month")
            path: "qrc:/timer-option-icons/1mo.svg"
        }

        ListElement {
            name: qsTr("1 year")
            path: "qrc:/timer-option-icons/1y.svg"
        }
    }

    Instantiator {
        model: timerModel

        MenuItem {
            text: name
            checkable: true
            checked: conversationItem.expirationPeriod === index
            onTriggered: conversationItem.expirationPeriod = index
        }

        onObjectAdded: optMenu.insertItem(index, object)
    }
}
