import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import "qrc:/imports/Avatar" as Av
import "qrc:/imports/js/utils.mjs" as Utils

Column {
    width: parent.width

    TextArea {
        id: groupSelectText
        leftPadding: 12
        placeholderText: "Add members"
    }

    Rectangle {
        height: 2
        width: parent.width - CmnCfg.largeMargin
        anchors.horizontalCenter: parent.horizontalCenter
        color: "black"
    }

    Item {
        width: parent.width
        height: 10
    }

    ListView {
        model: contactsModel
        width: parent.width
        height: 60
       // Layout.fillHeight: true
    delegate: Item {
        id: contactItem
        property var contactData: model

        height: CmnCfg.convoHeight
        width: parent.width
        visible: matched

        Common.PlatonicRectangle {
            id: contactRectangle
            boxColor: contactData.color
            boxTitle: contactData.name
            picture: Utils.safeStringOrDefault(contactData.profilePicture, "")

            labelComponent: Av.ConversationLabel {
                contactName: contactData.name
                labelColor: CmnCfg.palette.secondaryColor
                labelSize: 14
                lastBody: "@" + contactData.userId
            }
    }
}
}

}


