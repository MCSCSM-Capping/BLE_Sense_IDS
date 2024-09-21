import win32file, pywintypes

pipe_name = r"\\.\pipe\mypipe"

print(f"Connecting to named pipe: {pipe_name}")
pipe = win32file.CreateFileW(
    pipe_name,  # Pipe name in wide-character format
    win32file.GENERIC_WRITE,  # Write access
    0,
    None,
    win32file.OPEN_EXISTING,
    0,
    None,
)

print("Sending data to the pipe...")
data = "Hello from the client!"
win32file.WriteFile(pipe, data.encode("utf-8"))

print("Data sent. Closing pipe...")
win32file.CloseHandle(pipe)
