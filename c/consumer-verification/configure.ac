AC_INIT([consumer-verification], [0.0.0], [bug-automake@gnu.org])
AM_INIT_AUTOMAKE([-Wall -Werror foreign])
m4_include([libcurl.m4])
AC_PROG_CC
LIBCURL_CHECK_CONFIG([yes],[],[],[AC_MSG_ERROR([libcurl development files required])])
AC_CONFIG_HEADERS([config.h])
AC_CONFIG_FILES([
 Makefile
 src/Makefile
])
AC_OUTPUT
