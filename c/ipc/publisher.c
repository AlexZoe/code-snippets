#include <stdio.h>
#include <time.h>
#include <unistd.h>

#include "heartbeat.h"

int main() {
  HeartbeatPub* pub = create_publisher(PPS_1HZ);

  if (!pub) return 1;

  while (1) {
    publish(pub);
    sleep(1);
  }

  return 0;
}
