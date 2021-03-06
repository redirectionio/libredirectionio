AC_INIT([libredirectionio], [1.0.0], [contact@redirection.io])
AC_PREFIX_DEFAULT(/usr)

AC_PATH_PROG(CARGO_BIN, [cargo], [])
AS_IF(test x$CARGO_BIN = x,
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

AC_ARG_WITH([target_dir], [AS_HELP_STRING([--with-target-dir],
    [target directory])],
    [TARGET_DIR=$withval], [TARGET_DIR=])

AC_ARG_WITH([root_dir], [AS_HELP_STRING([--with-root-dir],
    [root directory of cargo])],
    [ROOT_DIR=$withval], [ROOT_DIR=])

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

AC_ARG_VAR([target_dir], [Target directory to use])

if test "x$TARGET_DIR" = "x" ; then
    RUST_TARGET_DIR=target
else
    RUST_TARGET_DIR=$TARGET_DIR
fi

AC_ARG_VAR([root_dir], [Root directory of cargo to use])

if test "x$ROOT_DIR" = "x" ; then
    RUST_ROOT_DIR=.
else
    RUST_ROOT_DIR=$ROOT_DIR
fi

AC_SUBST([CARGO_BIN])
AC_SUBST([RUST_RELEASE])
AC_SUBST([RUST_ROOT_DIR])
AC_SUBST([RUST_TARGET_DIR])
AC_SUBST([RUST_TARGET_SUBDIR])

AC_MSG_NOTICE([summary of build options:
    Install prefix:        ${prefix}
    Debugging information: ${debug_release}
])

AC_CONFIG_FILES([Makefile])
AC_CONFIG_FILES([libredirectionio.pc])

AC_OUTPUT
