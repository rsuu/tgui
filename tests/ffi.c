#include <android/api-level.h>
#include <dlfcn.h>
#include <stdio.h>

typedef struct AHardwareBuffer AHardwareBuffer;

struct LibAndroid {
  void *lib;
  void (*release)(AHardwareBuffer *);
  int (*recv)(int, AHardwareBuffer **);
};

static struct LibAndroid libandroid;

void LibAndroid_init() {
  libandroid.lib = NULL;
  libandroid.release = NULL;
  libandroid.recv = NULL;

  if (android_get_device_api_level() < 26)
    return;

  libandroid.lib = dlopen("libandroid.so", RTLD_LAZY | RTLD_LOCAL);
  if (libandroid.lib != NULL) {
    libandroid.release = (void (*)(AHardwareBuffer *))dlsym(
        libandroid.lib, "AHardwareBuffer_release");
    libandroid.recv = (int (*)(int, AHardwareBuffer **))dlsym(
        libandroid.lib, "AHardwareBuffer_recvHandleFromUnixSocket");

    printf("OK\n");
    printf("%p\n", libandroid);
    printf("%p\n", libandroid.release);
    printf("%p\n", libandroid.recv);
  }
}

void LibAndroid_destroy() {
  if (libandroid.lib != NULL)
    dlclose(libandroid.lib);
  libandroid.lib = NULL;
  libandroid.release = NULL;
  libandroid.recv = NULL;
}

int main() {
  LibAndroid_init();

  // Main code here

  LibAndroid_destroy();

  return 0;
}
