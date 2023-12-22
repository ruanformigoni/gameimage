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

// Prototypes for the original functions
ssize_t (*original_read)(int, void *, size_t) = NULL;
int (*original_access)(const char *, int) = NULL;
int (*original_stat)(const char *, struct stat *) = NULL;
int (*original_open)(const char *, int, ...) = NULL;
int (*original_openat)(int, const char *, int, ...) = NULL;
ssize_t (*original_readlink)(const char *, char *, size_t) = NULL;

// read override
ssize_t read(int fd, void *buf, size_t count) {
    char path[1024];
    sprintf(path, "/proc/self/fd/%d", fd);
    char actualpath[1024];
    ssize_t len = readlink(path, actualpath, sizeof(actualpath));

    if (len > 0 && strncmp(actualpath, "/usr", 4) == 0) {
        return -1; // Block the read operation
    }

    if (!original_read) {
        original_read = dlsym(RTLD_NEXT, "read");
    }
    return original_read(fd, buf, count);
}

// access override
int access(const char *pathname, int mode) {
    if (strncmp(pathname, "/usr", 4) == 0) {
        return -1; // Block the access operation
    }

    if (!original_access) {
        original_access = dlsym(RTLD_NEXT, "access");
    }
    return original_access(pathname, mode);
}

// stat override
int stat(const char *pathname, struct stat *statbuf) {
    if (strncmp(pathname, "/usr", 4) == 0) {
        return -1; // Block the stat operation
    }

    if (!original_stat) {
        original_stat = dlsym(RTLD_NEXT, "stat");
    }
    return original_stat(pathname, statbuf);
}

// open override
int open(const char *pathname, int flags, ...) {
    if (strncmp(pathname, "/usr", 4) == 0) {
        /* printf("Block to open %s\n", pathname); */
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
    if (strncmp(pathname, "/usr", 4) == 0) {
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

// readlink override
ssize_t readlink(const char *path, char *buf, size_t bufsiz) {
    if (strncmp(path, "/usr", 4) == 0) {
        return -1; // Block the readlink operation
    }

    if (!original_readlink) {
        original_readlink = dlsym(RTLD_NEXT, "readlink");
    }
    return original_readlink(path, buf, bufsiz);
}

// cmd: !gcc -fPIC -shared -o preload-sandbox.so % -ldl
