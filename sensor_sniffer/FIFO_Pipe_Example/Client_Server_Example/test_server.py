import win32pipe, win32file, pywintypes

pipe_name = r"\\.\pipe\mypipe"

# Create the named pipe with wide-character (W) support
print(f"Creating named pipe: {pipe_name}")
pipe = win32pipe.CreateNamedPipe(
    pipe_name,  # Pipe name in wide-character format
    win32pipe.PIPE_ACCESS_DUPLEX,  # Read/Write access
    win32pipe.PIPE_TYPE_MESSAGE
    | win32pipe.PIPE_READMODE_MESSAGE
    | win32pipe.PIPE_WAIT,  # Message type pipe, blocking
    1,
    65536,
    65536,
    0,
    None,  # Max instances, output buffer, input buffer, timeout
)

print("Waiting for a client to connect...")
win32pipe.ConnectNamedPipe(pipe, None)

try:
    print("Client connected, waiting for data...")
    while True:
        # Read up to 64 KB of data from the pipe
        result, data = win32file.ReadFile(pipe, 64 * 1024)
        print(f"Received: {data.decode('utf-8')}")
except pywintypes.error as e:
    print(f"Error: {e}")
finally:
    print("Closing pipe...")
    win32file.CloseHandle(pipe)
