#!/usr/bin/env bash

if [ ${EUID} -ne 0 ]; then
	echo "This script must be run as root. Use sudo or switch to the root user."
	exit 1
fi

if [ ! -f monsetor ]; then
    echo "Error: monsetor script not found in current directory."
    exit 1
fi

cp monsetor /usr/local/bin/
chmod +x /usr/local/bin/monsetor

echo "monsetor has been installed."
echo "Run 'monsetor --help' to see available options."
