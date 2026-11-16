#!/bin/bash
clear
echo -e "\033[1;36m========================================================================\033[0m"
echo -e "\033[1;37m                 APT / APTITUDE PACKAGE MANAGEMENT CONSOLE               \033[0m"
echo -e "\033[1;36m========================================================================\033[0m"
echo "1. Search for a package"
echo "2. Install a package (via Aptitude solver)"
echo "3. Update repository package lists (apt update)"
echo "4. List installed packages"
echo "5. Launch interactive Aptitude visual UI"
echo "6. Exit"
echo ""
read -p "Select option [1-6]: " opt

case $opt in
    1)
        read -p "Enter package name to search: " pkg
        apt-cache search "$pkg" | head -n 25
        ;;
    2)
        read -p "Enter package name to install: " pkg
        sudo aptitude install "$pkg"
        ;;
    3)
        sudo aptitude update
        ;;
    4)
        dpkg --get-selections | head -n 30
        ;;
    5)
        sudo aptitude
        ;;
    *)
        echo "Exiting."
        ;;
esac

echo ""
echo "Press [Enter] to close..."
read -r
