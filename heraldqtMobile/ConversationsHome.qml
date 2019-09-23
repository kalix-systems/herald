import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:///../heraldqtCommon" as HeraldQtCommon
Page {
  header: HeraldQtCommon.UtilityBar {
   buttonSpacing: 45
   marginWidth: QmlCfg.margin
   iconDimLarge: 45
   iconDimSmall: 30
   toolBarHeight: 70
   secondary: "light blue"
  }
}
