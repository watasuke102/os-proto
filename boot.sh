#!/usr/bin/env bash
set -e
BUILD_DIR="build"
DISK_IMG="${BUILD_DIR}/disk.img"
MOUNT_DIR="${BUILD_DIR}/mnt"
EFI_DIR="${MOUNT_DIR}/EFI/BOOT"

[ -e ${DISK_IMG} ] && rm -f ${DISK_IMG}

qemu-img create -f raw ${DISK_IMG} 100M
mkfs.fat ${DISK_IMG}

[ ! -e ${MOUNT_DIR} ] && mkdir -p ${MOUNT_DIR}
sudo mount -o loop ${DISK_IMG} ${MOUNT_DIR}
[ ! -e ${EFI_DIR} ] && sudo mkdir -p ${EFI_DIR}
sudo cp ${BUILD_DIR}/loader.efi ${EFI_DIR}/BOOTX64.EFI
sudo cp ${BUILD_DIR}/kernel.elf ${MOUNT_DIR}
sudo umount ${MOUNT_DIR}

qemu-system-x86_64 -s -monitor stdio \
  -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
  -drive if=pflash,format=raw,file=OVMF_VARS.fd \
  -drive format=raw,file=${DISK_IMG} \
  #-d int,cpu_reset
