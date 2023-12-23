/**
* @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
* @file        : preload-sandbox
* @created     : Wednesday Dec 20, 2023 08:13:18 -03
*/


#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <stdarg.h>
#include <stdlib.h>
#include <limits.h>
#include <string.h>
#include <stdlib.h>

// Define a path for the debug log file
#define DEBUG_LOG_FILE "/tmp/gameimage/preload-sandbox.log"

// Define a macro for conditional debug printing to a file
#define DEBUG_PRINT(...) do { \
  if(getenv("GIMG_PRELOAD_DEBUG")) { \
    FILE *file = fopen(DEBUG_LOG_FILE, "a"); \
    if(file) { \
      fprintf(file, __VA_ARGS__); \
      fclose(file); \
    } \
  } \
} while(0)

// Redirect paths
#define REPLACEMENT_PATH "/tmp/gameimage"

char *redirect_path(const char *original) {
    DEBUG_PRINT("%s: Path %s\n", __FUNCTION__, original);
    const char *prefix = "/usr";
    if (strncmp(original, prefix, strlen(prefix)) == 0) {
        static char new_path[PATH_MAX];
        snprintf(new_path, sizeof(new_path), "%s%s", REPLACEMENT_PATH, original + strlen(prefix));
        DEBUG_PRINT("%s: Redirect %s\n", __FUNCTION__, new_path);
        return new_path;
    }
    return (char *)original; // Return the original if no replacement is needed
}

// Prototypes for the original functions
typedef ssize_t (*readlink_func_t)(const char *, char *, size_t);
ssize_t (*original_readlink)(const char *, char *, size_t) = NULL;

typedef int (*access_func_t)(const char *, int);
int (*original_access)(const char *, int) = NULL;

typedef int (*stat_func_t)(const char *, struct stat *);
int (*original_stat)(const char *, struct stat *) = NULL;
typedef int (*lstat_func_t)(const char *pathname, struct stat *statbuf);
int (*original_lstat)(const char *pathname, struct stat *statbuf) = NULL;

typedef int (*open_func_t)(const char *pathname, int flags, ...);
int (*original_open)(const char *pathname, int flags, ...) = NULL;
typedef int (*openat_func_t)(int, const char *, int, ...);
int (*original_openat)(int, const char *, int, ...) = NULL;


// readlink override
ssize_t readlink(const char *path, char *buf, size_t bufsiz)
{
  path = redirect_path(path);

  if (buf && strncmp(path, "/usr", 4) == 0)
  {
    DEBUG_PRINT("%s: BBlock %s\n", __FUNCTION__, buf);
    return -1; // Block the readlink operation
  }

  if (strncmp(path, "/usr", 4) == 0)
  {
    DEBUG_PRINT("%s: PBlock %s\n", __FUNCTION__, path);
    return -1; // Block the readlink operation
  }

  if (!original_readlink)
  {
    original_readlink = (readlink_func_t) dlsym(RTLD_NEXT, "readlink");
  }

  return original_readlink(path, buf, bufsiz);
}


// access override
int access(const char *pathname, int mode) {
  pathname = redirect_path(pathname);
  if (strncmp(pathname, "/usr", 4) == 0) {
    DEBUG_PRINT("%s: Block %s\n", __FUNCTION__, pathname);
    return -1; // Block the access operation
  }

  if (!original_access) {
    original_access = (access_func_t) dlsym(RTLD_NEXT, "access");
  }
  return original_access(pathname, mode);
}

// stat override
int stat(const char *pathname, struct stat *statbuf) {
  pathname = redirect_path(pathname);
  if (strncmp(pathname, "/usr", 4) == 0) {
    DEBUG_PRINT("%s: Block %s\n", __FUNCTION__, pathname);
    return -1; // Block the stat operation
  }

  if (!original_stat) {
    original_stat = (stat_func_t) dlsym(RTLD_NEXT, "stat");
  }
  return original_stat(pathname, statbuf);
}


int lstat(const char *pathname, struct stat *statbuf) {
  pathname = redirect_path(pathname);
  if (strncmp(pathname, "/usr", 4) == 0) {
    DEBUG_PRINT("%s: Block %s\n", __FUNCTION__, pathname);
    return -1; // Block access to /usr
  }

  if (!original_lstat) {
    original_lstat = (lstat_func_t) dlsym(RTLD_NEXT, "lstat");
  }

  return original_lstat(pathname, statbuf);
}

// open override
int open(const char *pathname, int flags, ...) {
  pathname = redirect_path(pathname);
  if (strncmp(pathname, "/usr", 4) == 0) {
    DEBUG_PRINT("Block to open %s\n", pathname);
    return -1; // Block the open operation
  }

  mode_t mode = 0;

  // Extract the mode (if present)
  if (flags & O_CREAT) {
    va_list arg;
    va_start(arg, flags);
    mode = va_arg(arg, mode_t);
    va_end(arg);
  }

  if (!original_open) {
    original_open = (open_func_t) dlsym(RTLD_NEXT, "open");
  }
  if (flags & O_CREAT) {
    return original_open(pathname, flags, mode);
  } else {
    return original_open(pathname, flags);
  }
}

// openat override
int openat(int dirfd, const char *pathname, int flags, ...) {
  pathname = redirect_path(pathname);
  if (strncmp(pathname, "/usr", 4) == 0) {
    DEBUG_PRINT("Block %s\n", __FUNCTION__);
    return -1; // Block the openat operation
  }

  mode_t mode = 0;

  // Extract the mode (if present)
  if (flags & O_CREAT) {
    va_list arg;
    va_start(arg, flags);
    mode = va_arg(arg, mode_t);
    va_end(arg);
  }

  if (!original_openat) {
    original_openat = (openat_func_t) dlsym(RTLD_NEXT, "openat");
  }

  if (flags & O_CREAT) {
    return original_openat(dirfd, pathname, flags, mode);
  } else {
    return original_openat(dirfd, pathname, flags);
  }
}

// cmd: !gcc -fPIC -shared -o preload-sandbox.so % -ldl
/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
