# matar (kill process for linux)

Matar is a high-performance process termination utility designed for Linux environments.
It implements advanced PID filtering to ensure system stability by automatically
excluding system-critical PIDs (<= 1000), the current process, and its parent.

Using a safety-first approach, it performs a two-stage cleaning process:
an initial SIGKILL followed by a verification pass to handle persistent or
zombie processes, ensuring a clean system state without accidental 'self-suicide'.

``` 
┌─[claudio@CachyOS] - [~/Documents/Rust/projects/matar] - [ter abr 07, 09:58]
└─[$] <git:(master)> matar --help
A robust process termination utility.

Usage: matar [OPTIONS] <TARGET>

Arguments:
  <TARGET>  The name or pattern of the process to terminate

Options:
  -f, --fast     Do not perform the second 'deep clean' pass
  -h, --help     Print help
  -V, --version  Print version
```
