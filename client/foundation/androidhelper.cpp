#include "androidhelper.h"



#ifdef Q_OS_ANDROID
#include <QAndroidJniObject>
#include <QAndroidIntent>
#include <QAndroidActivityResultReceiver>
#include <QUrl>
#include <QtAndroid>
#include <QColor>

AndroidHelper::AndroidHelper()
{

}

void AndroidHelper::set_status_bar_color(QColor color) {
  QtAndroid::runOnAndroidThread([=]() {
  QAndroidJniObject window = QtAndroid::androidActivity().callObjectMethod("getWindow", "()Landroid/view/Window;");
  window.callMethod<void>("addFlags", "(I)V",  0x80000000); // draw system bar back ground
  window.callMethod<void>("clearFlags", "(I)V", 0x04000000); // translucence flag low
  window.callMethod<void>("setStatusBarColor", "(I)V", color.rgba());
  });
}

//void AndroidHelper::send_notification(QString content) {
//    QAndroidJniObject javaNotification = QAndroidJniObject::fromString(content);
//    QAndroidJniObject::callStaticMethod<void>("org/qtproject/notification/NotificationBuilder", "notify", "(Ljava/lang/String;)V", javaNotification.object<jstring>());
//}

void AndroidHelper::launch_camera_dialog() {
// QAndroidJniObject::callStaticMethod<void>("org/qtproject/notification/NotificationBuilder", "open_gallery", "(V)String");
}

void AndroidHelper::launch_file_picker() {

}

void AndroidHelper::save_file_to_documents(QString fname) {

}

void AndroidHelper::save_file_to_gallery(QString fname) {

}

int AndroidHelper::resolve_content_url(QString content_url) {
  auto content_path =  QAndroidJniObject::fromString(content_url).object<jstring>();
  auto ctx =  QtAndroid::androidContext();
  return QAndroidJniObject::callStaticMethod<jint>("org/qtproject/notification/NotificationBuilder", "resolve_uri", "(Landroid/content/Context;Ljava/lang/String;)I", &ctx, content_path);
}

#endif
