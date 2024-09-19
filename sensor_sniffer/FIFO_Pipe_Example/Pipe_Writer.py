import sys
import win32file, pywintypes

pipe_name = r'\\.\pipe\mypipe'

# Create or connect to the named pipe
pipe = win32file.CreateFileW(
    pipe_name,  # Pipe name in wide-character format
    win32file.GENERIC_WRITE,  # Write access
    0, None,
    win32file.OPEN_EXISTING,
    0, None
)

try:
    while True:
        # Read from stdin (this is the piped output from nrfutil)
        data = sys.stdin.buffer.read(64 * 1024)
        if not data:
            break

        # Write to the named pipe
        win32file.WriteFile(pipe, data)
        sys.stdout.flush()  # Ensure output is flushed
finally:
    win32file.CloseHandle(pipe)
