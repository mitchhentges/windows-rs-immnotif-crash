# windows-rs interface access violation bug

I looked into this error when my `IMMNotificationClient` was causing crashes really intermittently (and the
simpler I made my code, the more infrequent the crashes :/)
Fortunately, it fails consistently under WinDbg, and this repo contains the smallest case that
causes the crash.

How to reproduce:

1. `cargo build`
2. Install WinDbg ([I do it from the Windows Store](https://www.microsoft.com/store/productId/9PGJGD53TN86))
3. Open WinDbg, do `File > Start debugging > Launch executable > path_to_this_repo\target\debug\windows-rs-immnotif-crash.exe`
4. Press the top-left green `Go` button once, as WinDbg sets an "initial breakpoint" that you have to skip past
5. To trigger the exception, we need to enable or disable an audio device:
   1. Open the Control Panel
   2. Open `Sound`
   3. Right click on a Playback or Recording device and enable/disable it
   4. WinDbg should suddenly "stop" and `View > Command` should show something like:

```
(153c.23ec): Access violation - code c0000005 (first chance)
First chance exceptions are reported before any exception handling.
This exception may be expected and handled.
MMDevApi!CDeviceEnumerator::OnDeviceStateChanged+0xe6:
00007ffa`5b7339d6 488b4018        mov     rax,qword ptr [rax+18h] ds:abababab`abababc3=????????????????
```
