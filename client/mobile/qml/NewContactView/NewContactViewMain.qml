import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Common" as Common
import "Controls"

Page {

    header: NewContactHeader {}

    Column {
        anchors.fill: parent
        Label {}
        TextArea {}
        Button {}
        TextArea {}
    }
}
