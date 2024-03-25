# UART Hacking
A universal asynchronous receiver transmitter (UART) is a computer hardware standard for asynchronous serial communication in which data is encoded in a serial format and transmitted at configurable speed. The following tools are aimed at the hardware Security Tester and a written in Rust. 'sbrute' is login brute-forcing tool for UART. Most UART connected are used to login to an embedded system. This tool is designed to allow a user to brute force the UART authentication process.
```
$ ./sbrute -h
sbrute v1.0
github.com/MY7H404
A Rust-based tool for serial brute force attacks

USAGE:
    sbrute [OPTIONS]

OPTIONS:
    -b, --baudrate <baudrate>
            The baud rate to be used [default: 115200]

    -d, --device <device>
            The serial device to connect to [default: /dev/ttyUSB0]

    -f, --passwordfile <passwordfile>
            The password file to be used [default: pass.txt]

    -h, --help
            Show this help message and exit

    -l, --usernameprompt <usernameprompt>
            String that defines the username login prompt [default: Login:]

    -p, --passwordprompt <passwordprompt>
            String that defines the login password prompt [default: Password:]

    -s, --loginsuccessstring <loginsuccessstring>
            String that defines a successful login [default: root@localhost$]

    -t, --timeout <timeout>
            Timeout value for the serial connection [default: 1]

    -u, --username <username>
            The username to be used for brute forcing [default: root]

    -v, --verbose
            Verbose reporting [default: OFF]

    -V, --version
            Print version information
```
