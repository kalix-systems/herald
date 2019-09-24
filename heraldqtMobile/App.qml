import QtQuick 2.0
import QtQuick.Controls 2.13

Page {
 id: appRoot
 anchors.fill: parent



 StackView {
  id: rootStackView
  anchors.fill: parent
  initialItem: ConversationsHome {}
 }


}
