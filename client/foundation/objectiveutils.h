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
  Q_INVOKABLE QString launch_file_picker();
signals:
   void chosen_file(QString value);

#endif
};

#endif // OBJECTIVEUTILS_H
