import socket
import struct

def test():
    with socket.create_connection(("127.0.0.1", 5591)) as sock:
        while True:
            command = input("> ")
            command_bytes = bytes(command, 'utf-8').decode('unicode_escape')
            command_size = len(command_bytes)

            size_bytes = struct.pack('>Q', command_size)

            sock.sendall(size_bytes)

            sock.sendall(command_bytes.encode())

            response = sock.recv(1024)
            print(response.decode('utf-8').encode("unicode_escape"))

test()