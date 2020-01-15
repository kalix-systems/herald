import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.12
import LibHerald 1.0

Page {

    ColumnLayout {
        anchors.fill: parent

        Label {
            Layout.alignment: Qt.AlignCenter
            id: instructions
            text: qsTr("Some Instructions")
        }

        Label {
            Layout.alignment: Qt.AlignCenter
            id: genText
            text: "Generated Text"
            background: Rectangle {
                border.color: CmnCfg.palette.offBlack
                height: parent.height + CmnCfg.defaultMargin
                width: parent.width + CmnCfg.defaultMargin
                anchors.centerIn: parent
            }
        }

        RowLayout {
            Layout.alignment: Qt.AlignCenter
            Button {
                id: cancelButton
                text: qsTr("Cancel")
                onClicked: loginPageLoader.sourceComponent = lpMain
            }
            Button {
                id: submitButton
                text: qsTr("Submit")
            }
        }
    }
}
