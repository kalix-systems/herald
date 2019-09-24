#include "d.h"

d::d(QObject *parent)
    : QAbstractItemModel(parent)
{
}

QVariant d::headerData(int section, Qt::Orientation orientation, int role) const
{
  // FIXME: Implement me!
}

QModelIndex d::index(int row, int column, const QModelIndex &parent) const
{
  // FIXME: Implement me!
}

QModelIndex d::parent(const QModelIndex &index) const
{
  // FIXME: Implement me!
}

int d::rowCount(const QModelIndex &parent) const
{
  if (!parent.isValid())
    return 0;

  // FIXME: Implement me!
}

int d::columnCount(const QModelIndex &parent) const
{
  if (!parent.isValid())
    return 0;

  // FIXME: Implement me!
}

QVariant d::data(const QModelIndex &index, int role) const
{
  if (!index.isValid())
    return QVariant();

  // FIXME: Implement me!
  return QVariant();
}
