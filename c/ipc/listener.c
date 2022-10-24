#include <nng/nng.h>
#include <nng/protocol/pubsub0/pub.h>
#include <nng/protocol/pubsub0/sub.h>
#include <stdio.h>
#include <unistd.h>

#define IPC_ADDR "ipc:///tmp/heartbeat.ipc"

int main() {
  int ret;
  nng_socket sock;
  char* buf = NULL;
  size_t size;
  char message_id[] = "deadbeef";

  if ((ret = nng_sub0_open(&sock)) != 0) {
    printf("open failed\n");
    return 1;
  }

  // Subscribe topic
  if ((ret = nng_socket_set(sock, NNG_OPT_SUB_SUBSCRIBE, message_id,
                            sizeof(message_id))) != 0) {
    printf("socket set failed\n");
    return 1;
  }

  // Subscribe all
/*
  if ((ret = nng_socket_set(sock, NNG_OPT_SUB_SUBSCRIBE, "", 0)) != 0) {
    printf("socket set failed\n");
    return 1;
  }
*/

  // Set timeout
  if (nng_socket_set_ms(sock, NNG_OPT_RECVTIMEO, 200) != 0) {
    printf("socket set failed\n");
    return 1;
  }

  if ((ret = nng_dial(sock, IPC_ADDR, NULL, 0)) != 0) {
    printf("dial failed\n");
    return 1;
  }

/*
  // Library-allocated buffer
  while (1) {
    ret = nng_recv(sock, &buf, &size, NNG_FLAG_ALLOC);
    if (ret != 0) {
      printf("timeout\n");
      continue;
    }
    printf("received\n");
    for (int i = 0; i < size; ++i) {
      printf("val[%i]: %i\n", i, (int)buf[i]);
    }
    nng_free(buf, size);
  }
*/

  //Pre-allocated buffer (message size + terminator)
  char local_buf[9] = {0};
  while (1) {
    ret = nng_recv(sock, local_buf, &size, 0);
    if (ret != 0) {
      printf("timeout\n");
      continue;
    }
    printf("received\n");
    for (int i = 0; i < size; ++i) {
      printf("val[%i]: %i\n", i, (int)local_buf[i]);
    }
  }

  return 0;
}
