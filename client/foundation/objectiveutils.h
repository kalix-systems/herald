#ifndef OBJECTIVEUTILS_H
#define OBJECTIVEUTILS_H
#include <QObject>
#include <QColor>
#include <QString>

class ObjectiveUtils : public QObject
{
  Q_OBJECT

public:
  ObjectiveUtils();
#ifdef Q_OS_IOS
  Q_INVOKABLE static void set_status_bar_color(QColor color);
  Q_INVOKABLE static void request_notifications();
  Q_INVOKABLE void launch_file_picker();
  Q_INVOKABLE void launch_camera_dialog();
signals:
   void fileChosen(QString filename);

#endif
};

#endif // OBJECTIVEUTILS_H
