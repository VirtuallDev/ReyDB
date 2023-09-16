import socket

def test():


    with socket.create_connection(("127.0.0.1", 5591)) as sock:
        while True:
            command = bytes(input("> "), 'utf-8').decode('unicode_escape')
            sock.send(command.encode())
            print(sock.recv(1024))

test()