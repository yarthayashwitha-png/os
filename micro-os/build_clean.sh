#!/usr/bin/env bash
set -e

# ==============================================================================
# AetherOS Micro: Headless Automated Live Hybrid ISO Build Script
# Debian 12 (Bookworm) Live Hybrid ISO
# Exact visual match to reference desktop:
#   - Solid Slate Desktop Background
#   - Windows-style Bottom Panel with "Start" Menu button
#   - Complete 10 Desktop Application Icons
#   - Pre-installed Research Lab, Snake, Aptitude, Wi-Fi & Dashboard
# ==============================================================================

export DEBIAN_FRONTEND=noninteractive
BUILD_DIR="/var/tmp/aether-os-build"
CHROOT_DIR="${BUILD_DIR}/chroot"
LIVE_DIR="${BUILD_DIR}/iso"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="${SCRIPT_DIR}/../build"
mkdir -p "${OUTPUT_DIR}"

echo "========================================================================"
echo " ==> [1/7] Cleaning previous workspaces..."
echo "========================================================================"
for m in dev/pts dev proc sys run; do
    if mountpoint -q "${CHROOT_DIR}/${m}" 2>/dev/null; then
        umount -lf "${CHROOT_DIR}/${m}" || true
    fi
done
rm -rf "${BUILD_DIR}"
mkdir -p "${CHROOT_DIR}" "${LIVE_DIR}/live" "${LIVE_DIR}/isolinux"

echo "========================================================================"
echo " ==> [2/7] Bootstrapping Debian 12 Bookworm (minbase amd64)..."
echo "========================================================================"
debootstrap --variant=minbase --arch=amd64 bookworm "${CHROOT_DIR}" http://deb.debian.org/debian/

echo "========================================================================"
echo " ==> [3/7] Mounting virtual and kernel filesystems for chroot..."
echo "========================================================================"
mount --bind /dev "${CHROOT_DIR}/dev"
mount --bind /dev/pts "${CHROOT_DIR}/dev/pts"
mount -t proc proc "${CHROOT_DIR}/proc"
mount -t sysfs sysfs "${CHROOT_DIR}/sys"

echo "========================================================================"
echo " ==> [4/7] Installing Desktop Environment & Reference Visual Styling..."
echo "========================================================================"

# Staging helper scripts & desktop icons into chroot
mkdir -p "${CHROOT_DIR}/tmp/research-lab"
cp "${SCRIPT_DIR}/research-lab/research_lab.c" "${CHROOT_DIR}/tmp/research-lab/"
cp "${SCRIPT_DIR}/research-lab/Makefile" "${CHROOT_DIR}/tmp/research-lab/"

mkdir -p "${CHROOT_DIR}/tmp/config"
cp -r "${SCRIPT_DIR}/config/"* "${CHROOT_DIR}/tmp/config/"

cat << 'EOF' | chroot "${CHROOT_DIR}" /bin/bash
set -e
export DEBIAN_FRONTEND=noninteractive

# 1. Repositories with Non-Free Firmware
cat << 'REPO' > /etc/apt/sources.list
deb http://deb.debian.org/debian bookworm main contrib non-free non-free-firmware
deb http://security.debian.org/debian-security bookworm-security main contrib non-free non-free-firmware
deb http://deb.debian.org/debian bookworm-updates main contrib non-free non-free-firmware
REPO
apt-get update

# 2. Install Desktop Environment & Applications
apt-get install -y --no-install-recommends \
    linux-image-amd64 \
    live-boot \
    systemd-sysv \
    udev \
    dbus \
    sudo \
    curl \
    nano \
    htop \
    network-manager \
    network-manager-gnome \
    wireless-tools \
    wpasupplicant \
    firmware-iwlwifi \
    firmware-realtek \
    firmware-atheros \
    firmware-misc-nonfree \
    firmware-linux-free \
    xorg \
    xserver-xorg-video-all \
    lightdm \
    lightdm-autologin-greeter \
    xfce4 \
    xfce4-terminal \
    xfce4-whiskermenu-plugin \
    thunar \
    nsnake \
    build-essential \
    libncurses5-dev \
    libncursesw5-dev \
    aptitude

# 3. Install Helper Utilities
cp /tmp/config/micro-dashboard.sh /usr/local/bin/micro-dashboard
cp /tmp/config/boot-benchmark.sh /usr/local/bin/boot-benchmark
cp /tmp/config/apt-gui.sh /usr/local/bin/apt-gui
chmod +x /usr/local/bin/micro-dashboard /usr/local/bin/boot-benchmark /usr/local/bin/apt-gui

# 4. Build and Install Academic Research Lab
cd /tmp/research-lab
make
make install
rm -rf /tmp/research-lab

# 5. Configure User 'user' with Passwordless Sudo
id -u user &>/dev/null || useradd -m -s /bin/bash -G sudo,audio,video,netdev user
echo "user:user" | chpasswd
echo "root:root" | chpasswd
echo "user ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/user
chmod 0440 /etc/sudoers.d/user
echo "alias apt='aptitude'" >> /etc/bash.bashrc
echo "alias apt='aptitude'" >> /home/user/.bashrc

# 6. Configure Networking & LightDM Autologin
echo "micro-os" > /etc/hostname
mkdir -p /etc/lightdm/lightdm.conf.d
cat << 'LIGHTDM' > /etc/lightdm/lightdm.conf
[Seat:*]
autologin-user=user
autologin-user-timeout=0
user-session=xfce
greeter-session=lightdm-autologin-greeter
LIGHTDM

# 7. Symlink Snake Game
ln -sf /usr/games/nsnake /usr/local/bin/snake
ln -sf /usr/games/nsnake /usr/bin/snake

# 8. Deploy the Exact 10 Desktop Icons matching Reference Image
mkdir -p /home/user/Desktop /etc/skel/Desktop
cp /tmp/config/desktop-icons/*.desktop /home/user/Desktop/
cp /tmp/config/desktop-icons/*.desktop /etc/skel/Desktop/
chmod +x /home/user/Desktop/*.desktop /etc/skel/Desktop/*.desktop
chown -R user:user /home/user/Desktop

rm -rf /tmp/config
apt-get autoremove -y
apt-get clean
rm -rf /var/lib/apt/lists/*
rm -rf /usr/share/doc/* /usr/share/man/*
EOF

echo "========================================================================"
echo " ==> [5/7] Preparing static device nodes & unmounting chroot..."
echo "========================================================================"
mknod -m 600 "${CHROOT_DIR}/dev/console" c 5 1 || true
mknod -m 666 "${CHROOT_DIR}/dev/null" c 1 3 || true

VMLINUZ=$(ls -1 "${CHROOT_DIR}/boot"/vmlinuz-* | head -n 1)
INITRD=$(ls -1 "${CHROOT_DIR}/boot"/initrd.img-* | head -n 1)
cp "${VMLINUZ}" "${LIVE_DIR}/live/vmlinuz"
cp "${INITRD}" "${LIVE_DIR}/live/initrd.img"

umount -lf "${CHROOT_DIR}/sys" || true
umount -lf "${CHROOT_DIR}/proc" || true
umount -lf "${CHROOT_DIR}/dev/pts" || true
umount -lf "${CHROOT_DIR}/dev" || true

mkdir -p "${CHROOT_DIR}/proc" "${CHROOT_DIR}/sys" "${CHROOT_DIR}/dev" "${CHROOT_DIR}/run" "${CHROOT_DIR}/tmp"
chmod 1777 "${CHROOT_DIR}/tmp"

echo "========================================================================"
echo " ==> [6/7] Generating compressed SquashFS filesystem..."
echo "========================================================================"
mksquashfs "${CHROOT_DIR}" "${LIVE_DIR}/live/filesystem.squashfs" -comp xz -e boot

echo "========================================================================"
echo " ==> [7/7] Authoring bootable hybrid ISO..."
echo "========================================================================"
cp "${SCRIPT_DIR}/config/isolinux.cfg" "${LIVE_DIR}/isolinux/isolinux.cfg"
cp /usr/lib/ISOLINUX/isolinux.bin "${LIVE_DIR}/isolinux/" 2>/dev/null || cp /usr/lib/syslinux/isolinux.bin "${LIVE_DIR}/isolinux/"
cp /usr/lib/syslinux/modules/bios/* "${LIVE_DIR}/isolinux/" 2>/dev/null || true

xorriso -as mkisofs -r -V "MicroOS" \
    -J -l -b isolinux/isolinux.bin \
    -c isolinux/boot.cat \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    -isohybrid-mbr /usr/lib/ISOLINUX/isohdpfx.bin \
    -o "${OUTPUT_DIR}/aether-os.iso" "${LIVE_DIR}"

echo "========================================================================"
echo " SUCCESS! Exact replica ISO generated at: ${OUTPUT_DIR}/aether-os.iso"
echo "========================================================================"
