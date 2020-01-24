#ifndef USERMAP_H
#define USERMAP_H
#include "Bindings.h"
#include <QHash>

class UserMap : public QObject {
  Q_OBJECT
public:
  UserMap() {}
  /// get(uid), attempts to get a pointer to a user
  /// with id uid, if it does not exist, it is allocated and inserted
  Q_INVOKABLE QVariant get(const QString uid)
  {
    auto iter = userHash.find(uid);
    if (iter == userHash.end()) {
      // user does not exist
      auto user = new User();
      user->setUserId(uid);
      userHash.insert(uid, user);
      return QVariant::fromValue(user);
    }
    else {
      // user exists
      return QVariant::fromValue(iter.value());
    }
  }

private:
  QHash<QString, User*> userHash;
};

#endif // USERMAP_H
