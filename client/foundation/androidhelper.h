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
  Q_INVOKABLE static void send_notification(QString content);
  Q_INVOKABLE static QFile* open_gallery();
  Q_INVOKABLE static QFile* open_file_browser();
#endif
};

#endif // ANDROIDHELPERS_H
