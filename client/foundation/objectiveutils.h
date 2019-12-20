#ifndef OBJECTIVEUTILS_H
#define OBJECTIVEUTILS_H
#include <QObject>
#include <QColor>


class ObjectiveUtils : public QObject
{
  Q_OBJECT

public:
  ObjectiveUtils();
#ifdef Q_OS_IOS
  Q_INVOKABLE static void set_status_bar_color(QColor color);
  Q_INVOKABLE static void send_notification();
#endif
};

#endif // OBJECTIVEUTILS_H
