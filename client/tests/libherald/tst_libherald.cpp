#include "Bindings.h"
#include <QtTest>
// add necessary includes here

class libherald : public QObject
{
  Q_OBJECT

public:
  libherald();
  ~libherald();

private slots:
  void test_case1();

};

libherald::libherald()
{

}

libherald::~libherald()
{

}

void libherald::test_case1()
{

}

QTEST_APPLESS_MAIN(libherald)

#include "tst_libherald.moc"
