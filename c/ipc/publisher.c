#include <stdio.h>
#include <time.h>
#include <unistd.h>

#include "heartbeat.h"

int main() {
  HeartbeatPub* sub = create_publisher(PPS_1HZ);

  while (1) {
    publish(sub);
    sleep(1);
  }

  return 0;
}
