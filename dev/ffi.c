#include <dlfcn.h>
#include <stdio.h>

#include <android/api-level.h>
#include <android/hardware_buffer.h>

// bool AHardwareBuffer_set = false;
AHardwareBuffer *h_buffer;
AHardwareBuffer_Desc h_buffer_desc;

// typedef struct AHardwareBuffer AHardwareBuffer;

struct LibAndroid {
  void *lib;
  void (*release)(AHardwareBuffer *);
  int (*recv)(int, AHardwareBuffer **);
  int (*allocate)(AHardwareBuffer_Desc *, AHardwareBuffer **);

  int (*isSupported)(AHardwareBuffer_Desc *);
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

    libandroid.allocate =
        (int (*)(AHardwareBuffer_Desc *, AHardwareBuffer **))dlsym(
            libandroid.lib, "AHardwareBuffer_allocate");

    libandroid.isSupported = (int (*)(AHardwareBuffer_Desc *))dlsym(
        libandroid.lib, "AHardwareBuffer_isSupported");

    printf("OK\n");
  }
}

void LibAndroid_destroy() {
  if (libandroid.lib != NULL)
    dlclose(libandroid.lib);
  libandroid.lib = NULL;
  libandroid.release = NULL;
  libandroid.recv = NULL;
  libandroid.allocate = NULL;
}

int main() {
  LibAndroid_init();

  // test
  h_buffer_desc =
      (AHardwareBuffer_Desc){.stride = 0,
                             .height = 0,
                             .width = 0,
                             .layers = 1,
                             .format = AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM};

  // REFS: https://github.com/termux/termux-packages/wiki/Termux-and-Android-10
  //       https://github.com/termux/termux-app/issues/2155
  // int is_supported = AHardwareBuffer_isSupported(&h_buffer_desc);
  // int is_supported =  libandroid.is_supported(&h_buffer_desc, &h_buffer);
  // printf("%d\n", is_supported);

  libandroid.allocate(&h_buffer_desc, &h_buffer);

  LibAndroid_destroy();

  return 0;
}
