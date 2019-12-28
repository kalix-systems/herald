#include "androidhelper.h"

AndroidHelper::AndroidHelper()
{

}

#ifdef Q_OS_ANDROID
#include <QAndroidJniObject>
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

void AndroidHelper::send_notification(QString content) {
    QAndroidJniObject javaNotification = QAndroidJniObject::fromString(content);
    QAndroidJniObject::callStaticMethod<void>("org/qtproject/example/notification/NotificationBuilder", "notify", "(Ljava/lang/String;)V", javaNotification.object<jstring>());
}
#endif
