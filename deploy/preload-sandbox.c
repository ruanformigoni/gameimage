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

// Prototypes for the original functions
ssize_t (*original_readlink)(const char *, char *, size_t) = NULL;
ssize_t (*original_read)(int, void *, size_t) = NULL;

int (*original_access)(const char *, int) = NULL;

int (*original_stat)(const char *, struct stat *) = NULL;
int (*original_lstat)(const char *pathname, struct stat *statbuf) = NULL;

int (*original_open)(const char *pathname, int flags, ...) = NULL;

int (*original_openat)(int, const char *, int, ...) = NULL;


// readlink override
ssize_t readlink(const char *path, char *buf, size_t bufsiz)
{
  DEBUG_PRINT("%s: Try %s\n", __FUNCTION__, path);

  if (strncmp(path, "/usr", 4) == 0)
  {
    DEBUG_PRINT("%s: Block %s\n", __FUNCTION__, path);
    return -1; // Block the readlink operation
  }

  if (!original_readlink) {
    original_readlink = dlsym(RTLD_NEXT, "readlink");
  }

  return original_readlink(path, buf, bufsiz);
}


// read override
ssize_t read(int fd, void *buf, size_t count) {
  DEBUG_PRINT("%s: Read %s\n", __FUNCTION__, (char*) buf);

  char path[1024];
  sprintf(path, "/proc/self/fd/%d", fd);

  char actualpath[1024];
  ssize_t len = readlink(path, actualpath, sizeof(actualpath));

  DEBUG_PRINT("%s: Try %s\n", __FUNCTION__, actualpath);
  if (len > 0 && strncmp(actualpath, "/usr", 4) == 0) {
    DEBUG_PRINT("%s: Block %s\n", __FUNCTION__, actualpath);
    return -1; // Block the read operation
  }

  if (!original_read) {
    original_read = dlsym(RTLD_NEXT, "read");
  }
  return original_read(fd, buf, count);
}

// access override
int access(const char *pathname, int mode) {
  DEBUG_PRINT("%s: Try %s\n", __FUNCTION__, pathname);
  if (strncmp(pathname, "/usr", 4) == 0) {
    DEBUG_PRINT("%s: Block %s\n", __FUNCTION__, pathname);
    return -1; // Block the access operation
  }

  if (!original_access) {
    original_access = dlsym(RTLD_NEXT, "access");
  }
  return original_access(pathname, mode);
}

// stat override
int stat(const char *pathname, struct stat *statbuf) {
  DEBUG_PRINT("%s: Try %s\n", __FUNCTION__, pathname);
  if (strncmp(pathname, "/usr", 4) == 0) {
    DEBUG_PRINT("%s: Block %s\n", __FUNCTION__, pathname);
    return -1; // Block the stat operation
  }

  if (!original_stat) {
    original_stat = dlsym(RTLD_NEXT, "stat");
  }
  return original_stat(pathname, statbuf);
}


int lstat(const char *pathname, struct stat *statbuf) {
  DEBUG_PRINT("%s: Try %s\n", __FUNCTION__, pathname);
  if (strncmp(pathname, "/usr", 4) == 0) {
    DEBUG_PRINT("%s: Block %s\n", __FUNCTION__, pathname);
    return -1; // Block access to /usr
  }

  if (!original_lstat) {
    original_lstat = dlsym(RTLD_NEXT, "lstat");
  }

  return original_lstat(pathname, statbuf);
}

// open override
int open(const char *pathname, int flags, ...) {
  if (strncmp(pathname, "/usr", 4) == 0) {
    /* DEBUG_PRINT("Block to open %s\n", pathname); */
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
    original_open = dlsym(RTLD_NEXT, "open");
  }
  if (flags & O_CREAT) {
    return original_open(pathname, flags, mode);
  } else {
    return original_open(pathname, flags);
  }
}

// openat override
int openat(int dirfd, const char *pathname, int flags, ...) {
  DEBUG_PRINT("%s: Try %s\n", __FUNCTION__, pathname);
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
    original_openat = dlsym(RTLD_NEXT, "openat");
  }

  if (flags & O_CREAT) {
    return original_openat(dirfd, pathname, flags, mode);
  } else {
    return original_openat(dirfd, pathname, flags);
  }
}

// cmd: !gcc -fPIC -shared -o preload-sandbox.so % -ldl
/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
