#ifndef ANDROIDHELPERS_H
#define ANDROIDHELPERS_H
#include <QObject>
#include <QColor>
#include <QFile>

class AndroidHelper: public QObject
{
  Q_OBJECT
public:
  AndroidHelper();
#ifdef Q_OS_ANDROID
  Q_INVOKABLE static void set_status_bar_color(QColor color);
  Q_INVOKABLE static void save_file_to_gallery(QString fname);
  Q_INVOKABLE static void save_file_to_documents(QString fname);
  Q_INVOKABLE int resolve_content_url(QString content_url);
  Q_INVOKABLE void launch_file_picker();
  Q_INVOKABLE void launch_camera_dialog();
signals:
  void fileChosen(QString filename);
#endif
};

#endif // ANDROIDHELPERS_H
