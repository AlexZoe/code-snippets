#include <stdio.h>
#include <unistd.h>

#include <nng/nng.h>
#include <nng/protocol/pubsub0/pub.h>
#include <nng/protocol/pubsub0/sub.h>

#define IPC_ADDR "ipc:///tmp/heartbeat.ipc"

int main()
{
    int ret;
    nng_socket sock;
    if ((ret = nng_pub0_open(&sock)) != 0)
    {
        printf("open failed\n");
        return 1;
    }

    if ((ret = nng_listen(sock, IPC_ADDR, NULL, 0)) < 0)
    {
        printf("listen failed\n");
        return 1;
    }

    while (1)
    {
	char message_id[] = "deadbeef";
        ret = nng_send(sock, (void*)&message_id, (size_t)sizeof(message_id), 0);
        sleep(1);
    }

    return 0;
}
