/* generated by rust_qt_binding_generator */
#include "Bindings.h"

namespace {

    struct option_quintptr {
    public:
        quintptr value;
        bool some;
        operator QVariant() const {
            if (some) {
                return QVariant::fromValue(value);
            }
            return QVariant();
        }
    };
    static_assert(std::is_pod<option_quintptr>::value, "option_quintptr must be a POD type.");

    typedef void (*qstring_set)(QString* val, const char* utf8, int nbytes);
    void set_qstring(QString* val, const char* utf8, int nbytes) {
        *val = QString::fromUtf8(utf8, nbytes);
    }

    typedef void (*qbytearray_set)(QByteArray* val, const char* bytes, int nbytes);
    void set_qbytearray(QByteArray* v, const char* bytes, int nbytes) {
        if (v->isNull() && nbytes == 0) {
            *v = QByteArray(bytes, nbytes);
        } else {
            v->truncate(0);
            v->append(bytes, nbytes);
        }
    }

    struct qmodelindex_t {
        int row;
        quintptr id;
    };
    inline QVariant cleanNullQVariant(const QVariant& v) {
        return (v.isNull()) ?QVariant() :v;
    }
}
extern "C" {
    qint64 contacts_data_contact_id(const Contacts::Private*, int);
    void contacts_data_name(const Contacts::Private*, int, QString*, qstring_set);
    bool contacts_set_data_name(Contacts::Private*, int, const ushort* s, int len);
    void contacts_sort(Contacts::Private*, unsigned char column, Qt::SortOrder order = Qt::AscendingOrder);

    int contacts_row_count(const Contacts::Private*);
    bool contacts_insert_rows(Contacts::Private*, int, int);
    bool contacts_remove_rows(Contacts::Private*, int, int);
    bool contacts_can_fetch_more(const Contacts::Private*);
    void contacts_fetch_more(Contacts::Private*);
}
int Contacts::columnCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : 1;
}

bool Contacts::hasChildren(const QModelIndex &parent) const
{
    return rowCount(parent) > 0;
}

int Contacts::rowCount(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : contacts_row_count(m_d);
}

bool Contacts::insertRows(int row, int count, const QModelIndex &)
{
    return contacts_insert_rows(m_d, row, count);
}

bool Contacts::removeRows(int row, int count, const QModelIndex &)
{
    return contacts_remove_rows(m_d, row, count);
}

QModelIndex Contacts::index(int row, int column, const QModelIndex &parent) const
{
    if (!parent.isValid() && row >= 0 && row < rowCount(parent) && column >= 0 && column < 1) {
        return createIndex(row, column, (quintptr)row);
    }
    return QModelIndex();
}

QModelIndex Contacts::parent(const QModelIndex &) const
{
    return QModelIndex();
}

bool Contacts::canFetchMore(const QModelIndex &parent) const
{
    return (parent.isValid()) ? 0 : contacts_can_fetch_more(m_d);
}

void Contacts::fetchMore(const QModelIndex &parent)
{
    if (!parent.isValid()) {
        contacts_fetch_more(m_d);
    }
}
void Contacts::updatePersistentIndexes() {}

void Contacts::sort(int column, Qt::SortOrder order)
{
    contacts_sort(m_d, column, order);
}
Qt::ItemFlags Contacts::flags(const QModelIndex &i) const
{
    auto flags = QAbstractItemModel::flags(i);
    if (i.column() == 0) {
        flags |= Qt::ItemIsEditable;
    }
    return flags;
}

qint64 Contacts::contact_id(int row) const
{
    return contacts_data_contact_id(m_d, row);
}

QString Contacts::name(int row) const
{
    QString s;
    contacts_data_name(m_d, row, &s, set_qstring);
    return s;
}

bool Contacts::setName(int row, const QString& value)
{
    bool set = false;
    set = contacts_set_data_name(m_d, row, value.utf16(), value.length());
    if (set) {
        QModelIndex index = createIndex(row, 0, row);
        Q_EMIT dataChanged(index, index);
    }
    return set;
}

QVariant Contacts::data(const QModelIndex &index, int role) const
{
    Q_ASSERT(rowCount(index.parent()) > index.row());
    switch (index.column()) {
    case 0:
        switch (role) {
        case Qt::UserRole + 0:
            return QVariant::fromValue(contact_id(index.row()));
        case Qt::UserRole + 1:
            return QVariant::fromValue(name(index.row()));
        }
        break;
    }
    return QVariant();
}

int Contacts::role(const char* name) const {
    auto names = roleNames();
    auto i = names.constBegin();
    while (i != names.constEnd()) {
        if (i.value() == name) {
            return i.key();
        }
        ++i;
    }
    return -1;
}
QHash<int, QByteArray> Contacts::roleNames() const {
    QHash<int, QByteArray> names = QAbstractItemModel::roleNames();
    names.insert(Qt::UserRole + 0, "contact_id");
    names.insert(Qt::UserRole + 1, "name");
    return names;
}
QVariant Contacts::headerData(int section, Qt::Orientation orientation, int role) const
{
    if (orientation != Qt::Horizontal) {
        return QVariant();
    }
    return m_headerData.value(qMakePair(section, (Qt::ItemDataRole)role), role == Qt::DisplayRole ?QString::number(section + 1) :QVariant());
}

bool Contacts::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (orientation != Qt::Horizontal) {
        return false;
    }
    m_headerData.insert(qMakePair(section, (Qt::ItemDataRole)role), value);
    return true;
}

bool Contacts::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (index.column() == 0) {
        if (role == Qt::UserRole + 1) {
            if (value.canConvert(qMetaTypeId<QString>())) {
                return setName(index.row(), value.value<QString>());
            }
        }
    }
    return false;
}

extern "C" {
    Contacts::Private* contacts_new(Contacts*,
        void (*)(const Contacts*),
        void (*)(Contacts*),
        void (*)(Contacts*),
        void (*)(Contacts*, quintptr, quintptr),
        void (*)(Contacts*),
        void (*)(Contacts*),
        void (*)(Contacts*, int, int),
        void (*)(Contacts*),
        void (*)(Contacts*, int, int, int),
        void (*)(Contacts*),
        void (*)(Contacts*, int, int),
        void (*)(Contacts*));
    void contacts_free(Contacts::Private*);
    qint64 contacts_add(Contacts::Private*, const ushort*, int);
    qint64 contacts_add_with_profile_picture(Contacts::Private*, const ushort*, int, const char*, int);
    void contacts_clear(Contacts::Private*);
    void contacts_profile_picture(const Contacts::Private*, qint64, QByteArray*, qbytearray_set);
    bool contacts_remove(Contacts::Private*, qint64);
};

Contacts::Contacts(bool /*owned*/, QObject *parent):
    QAbstractItemModel(parent),
    m_d(nullptr),
    m_ownsPrivate(false)
{
    initHeaderData();
}

Contacts::Contacts(QObject *parent):
    QAbstractItemModel(parent),
    m_d(contacts_new(this,
        [](const Contacts* o) {
            Q_EMIT o->newDataReady(QModelIndex());
        },
        [](Contacts* o) {
            Q_EMIT o->layoutAboutToBeChanged();
        },
        [](Contacts* o) {
            o->updatePersistentIndexes();
            Q_EMIT o->layoutChanged();
        },
        [](Contacts* o, quintptr first, quintptr last) {
            o->dataChanged(o->createIndex(first, 0, first),
                       o->createIndex(last, 0, last));
        },
        [](Contacts* o) {
            o->beginResetModel();
        },
        [](Contacts* o) {
            o->endResetModel();
        },
        [](Contacts* o, int first, int last) {
            o->beginInsertRows(QModelIndex(), first, last);
        },
        [](Contacts* o) {
            o->endInsertRows();
        },
        [](Contacts* o, int first, int last, int destination) {
            o->beginMoveRows(QModelIndex(), first, last, QModelIndex(), destination);
        },
        [](Contacts* o) {
            o->endMoveRows();
        },
        [](Contacts* o, int first, int last) {
            o->beginRemoveRows(QModelIndex(), first, last);
        },
        [](Contacts* o) {
            o->endRemoveRows();
        }
)),
    m_ownsPrivate(true)
{
    connect(this, &Contacts::newDataReady, this, [this](const QModelIndex& i) {
        this->fetchMore(i);
    }, Qt::QueuedConnection);
    initHeaderData();
}

Contacts::~Contacts() {
    if (m_ownsPrivate) {
        contacts_free(m_d);
    }
}
void Contacts::initHeaderData() {
}
qint64 Contacts::add(const QString& name)
{
    return contacts_add(m_d, name.utf16(), name.size());
}
qint64 Contacts::add_with_profile_picture(const QString& name, const QByteArray& profile)
{
    return contacts_add_with_profile_picture(m_d, name.utf16(), name.size(), profile.data(), profile.size());
}
void Contacts::clear()
{
    return contacts_clear(m_d);
}
QByteArray Contacts::profile_picture(qint64 id) const
{
    QByteArray s;
    contacts_profile_picture(m_d, id, &s, set_qbytearray);
    return s;
}
bool Contacts::remove(qint64 id)
{
    return contacts_remove(m_d, id);
}
