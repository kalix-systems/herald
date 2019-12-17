#ifndef OBJECTIVEUTILS_H
#define OBJECTIVEUTILS_H
#include <QObject>


class ObjectiveUtils : public QObject
{
  Q_OBJECT

public:
  ObjectiveUtils();
  static void set_navbar_color();
};

#endif // OBJECTIVEUTILS_H
