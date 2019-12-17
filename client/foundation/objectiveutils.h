#ifndef OBJECTIVEUTILS_H
#define OBJECTIVEUTILS_H
#include <QObject>


class ObjectiveUtils : public QObject
{
  Q_OBJECT

public:
  ObjectiveUtils();
#ifdef Q_OS_IOS
  static void set_navbar_color();
#endif
};

#endif // OBJECTIVEUTILS_H
