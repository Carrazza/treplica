import socket
import time

def send_multicast_message(message, multicast_group, multicast_port):

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)


    ttl = 2
    sock.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_TTL, ttl)


    message_bytes = message.encode('utf-8')

    sock.sendto(message_bytes, (multicast_group, multicast_port))

    sock.close()

multicast_group = '239.0.0.125'
multicast_port = 12345


for i in range(1000):

    message = f'teste multicast {i}'
    send_multicast_message(message, multicast_group, multicast_port)
    time.sleep(5)
