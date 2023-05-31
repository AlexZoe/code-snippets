#ifndef HEARTBEAT_H
#define HEARTBEAT_H

#include <time.h>

typedef enum {
  PPS_1HZ,
  PPS_4HZ,
} HeartbeatTopic;

typedef enum {
  HB_OK,
  HB_TIMEOUT,
  HB_BAD_SUB,
} HeartbeatResult;

typedef struct HeartbeatConnection HeartbeatPub, HeartbeatSub;

HeartbeatPub* create_publisher(HeartbeatTopic topic);
void destroy_publisher(HeartbeatPub* publisher);
void publish(HeartbeatSub* publisher);

HeartbeatSub* create_subscriber(HeartbeatTopic topic);
void destroy_subscriber(HeartbeatSub* subscriber);
HeartbeatResult receive(HeartbeatSub* subscriber, struct timespec* ts);

#endif
