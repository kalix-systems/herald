import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls.Styles 1.4
import LibHerald 1.0
import "../common" as Common
import "../common/utils.js" as Utils

ChatBubbleForm {
     property color repBubCol: "blue"
     additionalContent: ReplyComponent {}
}
