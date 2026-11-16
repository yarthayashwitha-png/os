#!/bin/bash
# Micro OS Dashboard - System Status & Hardware Monitor
clear
echo -e "\033[1;36m========================================================================\033[0m"
echo -e "\033[1;37m                      AETHER-MICRO OS DASHBOARD                         \033[0m"
echo -e "\033[1;36m========================================================================\033[0m"
echo ""
echo -e "\033[1;32m[+] Operating System   :\033[0m $(cat /etc/os-release | grep PRETTY_NAME | cut -d= -f2 | tr -d '\"')"
echo -e "\033[1;32m[+] Kernel Version     :\033[0m $(uname -srmo)"
echo -e "\033[1;32m[+] System Uptime      :\033[0m $(uptime -p)"
echo -e "\033[1;32m[+] Hostname           :\033[0m $(hostname)"
echo ""
echo -e "\033[1;33m--- Memory & Swap Allocation ---\033[0m"
free -h
echo ""
echo -e "\033[1;33m--- Disk Storage & Live Overlay ---\033[0m"
df -h /
echo ""
echo -e "\033[1;33m--- CPU & Architecture ---\033[0m"
lscpu | grep -E "Model name|Architecture|CPU\(s\):|Thread" | head -n 4
echo ""
echo -e "\033[1;36m========================================================================\033[0m"
echo "Press [Enter] to exit dashboard or 'h' for live htop..."
read -r choice
if [ "$choice" = "h" ] || [ "$choice" = "H" ]; then
    htop
fi
