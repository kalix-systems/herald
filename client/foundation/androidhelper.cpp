#include "androidhelper.h"


AndroidHelper::AndroidHelper()
{

}

#ifdef Q_OS_ANDROID
#include <QtAndroidExtras/QAndroidJniObject>
#include <QtAndroid>
#include <QColor>



void AndroidHelper::set_status_bar_color(QColor color) {
  QtAndroid::runOnAndroidThread([=]() {
  QAndroidJniObject window = QtAndroid::androidActivity().callObjectMethod("getWindow", "()Landroid/view/Window;");
  window.callMethod<void>("addFlags", "(I)V",  0x80000000); // draw system bar back ground
  window.callMethod<void>("clearFlags", "(I)V", 0x04000000); // translucence flag low
  window.callMethod<void>("setStatusBarColor", "(I)V", color.rgba());
  });
}

void AndroidHelper::send_notification() {
    QAndroidJniObject javaNotification = QAndroidJniObject::fromString("GRAPPO");
    QAndroidJniObject::callStaticMethod<void>("org/qtproject/qt5/NotificationBuilder", "notify", "(Ljava/lang/String;)V", javaNotification.object<jstring>());
}
#endif
