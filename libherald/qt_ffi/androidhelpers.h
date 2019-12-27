#ifndef ANDROIDHELPERS_H
#define ANDROIDHELPERS_H

#include <QObject>

class AndroidHelpers : public QObject {
  Q_OBJECT
public:
  explicit AndroidHelpers(QObject* parent = nullptr);

signals:
};

#endif // ANDROIDHELPERS_H
