#include "heartbeat.h"

#include <nng/nng.h>
#include <nng/protocol/pubsub0/pub.h>
#include <nng/protocol/pubsub0/sub.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// Have to use same size
static const char PPS_1HZ_TOPIC[] = "PPS1";
static const char PPS_4HZ_TOPIC[] = "PPS4";

static const int PAYLOAD_INDEX = sizeof(PPS_1HZ_TOPIC) - 1;

struct HeartbeatConnection {
  nng_socket socket;
  char message[sizeof(PPS_1HZ_TOPIC) - 1 + sizeof(struct timespec)];
};

static const char* get_topic_id(HeartbeatTopic topic) {
  switch (topic) {
    case PPS_1HZ:
      return PPS_1HZ_TOPIC;
    case PPS_4HZ:
      return PPS_4HZ_TOPIC;
  }

  return NULL;
}

HeartbeatPub* create_publisher(HeartbeatTopic topic) {
  const char* topic_id = get_topic_id(topic);

  HeartbeatPub* pub = malloc(sizeof(HeartbeatPub));
  snprintf(pub->message, sizeof(pub->message), "%s", topic_id);

  if (nng_pub0_open(&(pub->socket)) != 0) {
    printf("open failed\n");
    goto exit_err;
  }

  char ipc_addr[256];
  snprintf(ipc_addr, sizeof(ipc_addr), "ipc:///tmp/%s.ipc", topic_id);
  if (nng_listen(pub->socket, ipc_addr, NULL, 0) < 0) {
    printf("listen failed\n");
    goto exit_err;
  }

  return pub;

exit_err:
  free(pub);
  return NULL;
}

void destroy_publisher(HeartbeatPub* publisher) {
  if (publisher) free(publisher);
}

void publish(HeartbeatPub* publisher) {
  if (!publisher) return;

  timespec_get((struct timespec*)&(publisher->message[PAYLOAD_INDEX]),
               TIME_UTC);
  nng_send(publisher->socket, (void*)&publisher->message,
           (size_t)sizeof(publisher->message), 0);
}

HeartbeatSub* create_subscriber(HeartbeatTopic topic) {
  char truncated_string_terminator[PAYLOAD_INDEX];
  const char* topic_id = get_topic_id(topic);

  HeartbeatSub* sub = malloc(sizeof(HeartbeatSub));

  if (nng_sub0_open(&sub->socket) != 0) {
    printf("open failed\n");
    goto exit_err;
  }

  memcpy(&truncated_string_terminator, topic_id,
         sizeof(truncated_string_terminator));

  // Subscribe topic
  if (nng_socket_set(sub->socket, NNG_OPT_SUB_SUBSCRIBE,
                     truncated_string_terminator,
                     sizeof(truncated_string_terminator)) != 0) {
    printf("socket set failed\n");
    goto exit_err;
  }

  // Set timeout
  if (nng_socket_set_ms(sub->socket, NNG_OPT_RECVTIMEO, 200) != 0) {
    printf("socket set failed\n");
    goto exit_err;
  }

  char ipc_addr[256];
  snprintf(ipc_addr, sizeof(ipc_addr), "ipc:///tmp/%s.ipc", topic_id);
  if (nng_dial(sub->socket, ipc_addr, NULL, NNG_FLAG_NONBLOCK) != 0) {
    printf("dial failed\n");
    goto exit_err;
  }

  return sub;

exit_err:
  free(sub);
  return NULL;
}

void destroy_subscriber(HeartbeatSub* subscriber) {
  if (subscriber) free(subscriber);
}

HeartbeatResult receive(HeartbeatSub* subscriber, struct timespec* ts) {
  if (!subscriber || !ts) return HB_BAD_SUB;

  size_t size;
  int ret = nng_recv(subscriber->socket, subscriber->message, &size, 0);
  if (ret != 0) return HB_TIMEOUT;

  memcpy(ts, &subscriber->message[PAYLOAD_INDEX], sizeof(struct timespec));
  return HB_OK;
}
