#include <stdio.h>
#include <time.h>

#include "heartbeat.h"

int main() {
  HeartbeatSub* sub = create_subscriber(PPS_1HZ);
  struct timespec ts;

  while (1) {
    HeartbeatResult result = receive(sub, &ts);
    if (result == HB_TIMEOUT) {
      printf("timeout\n");
    } else {
      printf("ts: %lu (s), %lu (ns)\n", ts.tv_sec, ts.tv_nsec);
    }
  }

  return 0;
}
