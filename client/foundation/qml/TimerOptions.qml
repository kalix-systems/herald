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
    property var builder
    property int chosenPeriod: {
        if (!messageModify) {

            return conversationItem.expirationPeriod
        }
        builder.expirationPeriod
                !== undefined ? builder.expirationPeriod : conversationItem.expirationPeriod
    }
    property string chosenTimer: timerModel[chosenPeriod].path
    property bool messageModify: false

    // TODO real icon
    property var timerModel: [{
            "name": qsTr("Off"),
            "path": messageModify ? "qrc:/timer-option-icons/blank-dark.svg" : "qrc:/timer-option-icons/off.svg"
        }, {
            "name": qsTr("30 seconds"),
            "path": messageModify ? "qrc:/timer-option-icons/30s-dark.svg" : "qrc:/timer-option-icons/30s.svg"
        }, {
            "name": qsTr("1 minute"),
            "path": messageModify ? "qrc:/timer-option-icons/1min-dark.svg" : "qrc:/timer-option-icons/1min.svg"
        }, {
            "name": qsTr("30 minutes"),
            "path": messageModify ? "qrc:/timer-option-icons/30min-dark.svg" : "qrc:/timer-option-icons/30min.svg"
        }, {
            "name": qsTr("1 hour"),
            "path": messageModify ? "qrc:/timer-option-icons/1h-dark.svg" : "qrc:/timer-option-icons/1h.svg"
        }, {
            "name": qsTr("12 hours"),
            "path": messageModify ? "qrc:/timer-option-icons/12h-dark.svg" : "qrc:/timer-option-icons/12h.svg"
        }, {
            "name": qsTr("1 day"),
            "path": messageModify ? "qrc:/timer-option-icons/1d-dark.svg" : "qrc:/timer-option-icons/1d.svg"
        }, {
            "name": qsTr("1 week"),
            "path": messageModify ? "qrc:/timer-option-icons/1w-dark.svg" : "qrc:/timer-option-icons/1w.svg"
        }, {
            "name": qsTr("1 month"),
            "path": messageModify ? "qrc:/timer-option-icons/1mo-dark.svg" : "qrc:/timer-option-icons/1mo.svg"
        }, {
            "name": qsTr("1 year"),
            "path": messageModify ? "qrc:/timer-option-icons/1y-dark.svg" : "qrc:/timer-option-icons/1y.svg"
        }]
    Instantiator {
        model: timerModel

        MenuItem {
            text: timerModel[index].name
            checkable: true
            checked: {
                if (!messageModify) {

                    return conversationItem.expirationPeriod === index
                }
                builder.expirationPeriod
                        === undefined ? (conversationItem.expirationPeriod
                                         === index) : builder.expirationPeriod === index
            }
            onTriggered: {

                if (!messageModify) {
                    chosenPeriod = index

                    return conversationItem.expirationPeriod = index
                }
                builder.setExpirationPeriod(index)
                chosenPeriod = index
            }
        }

        onObjectAdded: optMenu.insertItem(index, object)
    }
}
