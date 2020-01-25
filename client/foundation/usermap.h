#ifndef USERMAP_H
#define USERMAP_H
#include "Bindings.h"
#include <QHash>
#include <QQmlApplicationEngine>

class UserMap : public QObject {
  Q_OBJECT
public:
  UserMap() {}
  /// get(uid), attempts to get a pointer to a user
  /// with id uid, if it does not exist, it is allocated and inserted
  Q_INVOKABLE QVariant get(const QString uid)
  {
    auto iter = userHash.find(uid);

    if ((iter == userHash.end()) || (iter.value() == nullptr)) {
      // user does not exist
      auto user = new User();
      // save us from the GC
      QQmlEngine::setObjectOwnership(user, QQmlEngine::CppOwnership);

      user->setUserId(uid);
      userHash.insert(uid, user);
      return QVariant::fromValue(user);
    }
    else {
      return QVariant::fromValue(iter.value());
    }
  }

private:
  QHash<QString, User*> userHash;
};

#endif // USERMAP_H
