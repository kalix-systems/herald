#!/usr/bin/env zsh

if (( ${+HERALDQT} )) ; then
  prefix=$HERALDQT
else
  prefix=$(pwd)
fi

if [ "$1" = "r" ] || [ "$1" = "release" ] ; then
  release=""
fi

cd $prefix/../libherald

if ((${+release})) ; then 
  cargo build --release
  destdir="release"
  qmake_cfg="CONFIG+=qtquickcompiler"
else
  cargo build
  destdir="debug"
  qmake_cfg="CONFIG+=debug CONFIG+=qml_debug"
fi

cd $prefix/target/$destdir && qmake $prefix/heraldqt.pro $qmake_cfg && make qmake_all && make -j && ./heraldqt

