#!/bin/bash
clear
echo -e "\033[1;36m========================================================================\033[0m"
echo -e "\033[1;37m                 BOOT TIME BENCHMARK & SYSTEM ANALYSIS                  \033[0m"
echo -e "\033[1;36m========================================================================\033[0m"
echo ""
echo -e "\033[1;32m[*] Total System Boot Time:\033[0m"
systemd-analyze time 2>/dev/null || uptime -p
echo ""
echo -e "\033[1;33m[*] Component Boot Blame (Slowest to Fastest Services):\033[0m"
systemd-analyze blame 2>/dev/null | head -n 12 || echo "Running in live-boot memory mode."
echo ""
echo -e "\033[1;36m========================================================================\033[0m"
echo "Press [Enter] to return to desktop..."
read -r
