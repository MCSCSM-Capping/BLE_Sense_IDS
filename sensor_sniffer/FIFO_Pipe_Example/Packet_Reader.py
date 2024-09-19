''' LINUX '''

# import os

# pipe_path = 'packetPipe'

# # Check if the pipe exists, create it if it doesn't
# if not os.path.exists(pipe_path):
#     os.mkfifo(pipe_path)

# # Open the pipe and read data continuously
# with open(pipe_path, 'r') as pipe:
#     print("Waiting for data...")
#     while True:
#         # Read line-by-line
#         data = pipe.readline()
#         if data:
#             print(f"Received: {data.strip()}")


'''WIN'''
import win32pipe, win32file, win32event, pywintypes

pipe_name = r'\\.\pipe\mypipe' # windows uses a special IPC (Inter-Process Communication) path
print("Pipe: " + pipe_name)

def create_named_pipe(name):
    return win32pipe.CreateNamedPipe(
        name,
        win32pipe.PIPE_ACCESS_INBOUND,
        win32pipe.PIPE_TYPE_BYTE | win32pipe.PIPE_READMODE_BYTE | win32pipe.PIPE_WAIT,
        1, 65536, 65536, 0, None
    )

def main():
    pipe = create_named_pipe(pipe_name)
    print("Named pipe created, waiting for connection...")
    win32pipe.ConnectNamedPipe(pipe, None)
    print("Client connected, waiting for data...")

    try:
        while True:
            data = win32file.ReadFile(pipe, 64*1024)[1]
            if data:
                print(data.decode('utf-8'))  # Adjust as needed for binary data
            else:
                break
    finally:
        win32file.CloseHandle(pipe)

if __name__ == '__main__':
    main()

