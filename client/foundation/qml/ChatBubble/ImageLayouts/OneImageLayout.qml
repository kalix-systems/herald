import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

Row {
    id: wrapperRow

    height: image.height
    property var firstImage
    property var imageClickedCallBack: function () {
        throw "undefined callback"
    }
    property var dims: JSON.parse(Herald.utils.imageScaling(firstImage.path,
                                                            300))

    Image {
        id: image
        sourceSize.width: dims.width
        sourceSize.height: dims.height
        source: "file:" + firstImage.path
        fillMode: Image.PreserveAspectCrop
        asynchronous: true

        MouseArea {
            onClicked: wrapperRow.imageClickedCallBack(image.source)
            anchors.fill: parent
        }
    }
}
