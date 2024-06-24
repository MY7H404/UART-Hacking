# UART Hacking
A universal asynchronous receiver transmitter (UART) is a computer hardware standard for asynchronous serial communication in which data is encoded in a serial format and transmitted at configurable speed. The following tools are aimed at the hardware Security Tester and a written in Rust. 'sbrute' is login brute-forcing tool for UART. Most UART connected are used to login to an embedded system. This tool is designed to allow a user to brute force the UART authentication process.
```
$ ./sbrute -h
sbrute v1.1
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
## Contribution

Contributions to UART-Hacking are welcome. If you encounter issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## Disclaimer

This tool is created for educational and testing purposes only. The authors and contributors are not responsible for any illegal, unethical, or unauthorized use of this tool. Users are solely responsible for ensuring that their use of this tool complies with all applicable laws, regulations, and ethical standards.

Usage of this tool on systems or networks without explicit authorization is strictly prohibited. The authors and contributors disclaim any responsibility for any damage, loss of data, or other consequences resulting from the use of this tool.

By using this tool, you acknowledge that you have read, understood, and agree to abide by the terms of this disclaimer.

**Use responsibly and ethically.**

## License
This project is licensed under the [MIT License.](https://github.com/MY7H404/UART-Hacking/blob/main/LICENSE)
