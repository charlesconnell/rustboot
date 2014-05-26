BDIR		 = ./efi

include ../../common/Makefile

TARGET		?= x86_64-intel-linux

LDFLAGS		?= --oformat pei-x86-64 -pie -e efi_start --subsystem 10 

QEMU		?= qemu-system-x86_64

# TODO: fix dep-info target
MODS		?= \
	$(wildcard *.rs) $(wildcard */*.rs) \
	$(wildcard ../../common/kernel/*.rs) $(wildcard ../../common/kernel/*/*.rs) \
	$(wildcard ../../common/kernel/*/*/*.rs)


.PHONY: all run debug

all: $(BDIR)/boot.efi
	@wc -c $^

# Compile rustboot
$(BDIR)/main.o: ../../common/lib.rs $(MODS)
	$(RUSTC) $(RUSTCFLAGS) --dep-info $(@D)/main.d -L $(BDIR)--emit=obj $< --out-dir $(BDIR)

$(BDIR)/boot.efi: $(BDIR)/main.o
	$(LD) $(LDFLAGS) $^ -o $@ $(BDIR)/core.o

# running
run: all
	$(QEMU) -L $(OVMF_ROOT) -hda fat:boot/ -no-kvm -m 32

# debug: $(BDIR)/boot.efi
# ifeq ($(strip $(TMUX)),)
#         tmux new-session -d -s rustboot "$(QEMU) -L $(OVMF_ROOT) -hda fat:boot/ -no-kvm -m 32 -s -S"
#         tmux new-window -t rustboot:1 "$(GDB)"
#         tmux a -t rustboot
#         tmux kill-session -t rustboot
# else
#         tmux split-w "$(GDB); tmux kill-p"
#         $(QEMU) -fda $(BDIR)/floppy.img -m 32 -s -S
# endif
