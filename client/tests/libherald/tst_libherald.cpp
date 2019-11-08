#include "Bindings.h"
#include <QtTest>
#include <QSignalSpy>
// add necessary includes here

class libherald : public QObject
{
  Q_OBJECT

public:
    Config*     cfg    = nullptr;
    HeraldState*  herald_state = nullptr;
    libherald();
    ~libherald();

private slots:
  void test_config_set_name();
  void test_config_set_color();

};

libherald::libherald()
{
    QFile file("store.sqlite3");
    file.remove();
    herald_state = new HeraldState();
    herald_state -> registerNewUser("Boruto");
    herald_state -> setConfigInit(true);

}

libherald::~libherald() {}

void libherald::test_config_set_name()
{
    cfg = new Config();
    QSignalSpy spy(cfg, SIGNAL(nameChanged()));
    cfg -> setName("Alfalfa");
    QCOMPARE(cfg -> name(), "Alfalfa");
    QCOMPARE(spy.count(), 1);
}

void libherald::test_config_set_color() {
    cfg = new Config();
    QSignalSpy spy(cfg, SIGNAL(colorChanged()));
    cfg -> setColor(0);
    QCOMPARE(cfg -> color(), 0);
    QCOMPARE(spy.count(), 1);
}

QTEST_APPLESS_MAIN(libherald)

#include "tst_libherald.moc"
