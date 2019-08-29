AC_INIT([libredirectionio], [1.0.0], [contact@redirection.io])
AC_PREFIX_DEFAULT(/usr)

AC_CHECK_PROG(CARGO, [cargo], [yes], [no])
AS_IF(test x$CARGO = xno,
    AC_MSG_ERROR([cargo is required.  Please install the Rust toolchain from https://www.rust-lang.org/])
)

AC_CHECK_PROG(RUSTC, [rustc], [yes], [no])
AS_IF(test x$RUSTC = xno,
    AC_MSG_ERROR([rustc is required.  Please install the Rust toolchain from https://www.rust-lang.org/])
)

dnl Specify --enable-debug to make a development release.  By default,
dnl we build in public release mode.

AC_ARG_ENABLE(debug,
              AC_HELP_STRING([--enable-debug],
                             [Build Rust code with debugging information [default=no]]),
              [debug_release=$enableval],
              [debug_release=no])

AC_MSG_CHECKING(whether to build Rust code with debugging information)
if test "x$debug_release" = "xyes" ; then
    AC_MSG_RESULT(yes)
    RUST_RELEASE=
    RUST_TARGET_SUBDIR=debug
else
    AC_MSG_RESULT(no)
    RUST_RELEASE=--release
    RUST_TARGET_SUBDIR=release
fi

AC_SUBST([RUST_RELEASE])
AC_SUBST([RUST_TARGET_SUBDIR])

AC_MSG_NOTICE([summary of build options:
    Install prefix:        ${prefix}
    Debugging information: ${debug_release}
])

AC_CONFIG_FILES([Makefile])
AC_CONFIG_FILES([libredirectionio.pc])

AC_OUTPUT
