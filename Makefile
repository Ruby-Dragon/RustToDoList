 CC := cargo
 CFLAGS ?= build --release --manifest-path
 DFLAGS ?= build --manifest-path
 PREFIX ?= /usr/local
 BINDIR ?= bin
 BUILDDIR ?= rust_to_do_list/target/release
 MANDIR := ./rust_to_do_list/Cargo.toml
 DESTDIR ?=
 LIST ?= storedtodolist.lst
 
 SUPPRESS := 2>/dev/null
 
 RM = rm
 CD = cd
 
 BINARY := rust_to_do_list
 FINBINARY := todolist
 
 all: $(BINARY)
 
 $(BINARY):
	$(CC) $(CFLAGS) $(MANDIR) $(SUPPRESS)
 
 install: $(BINARY)
	install -Dm0755 $(BUILDDIR)/$(BINARY) $(DESTDIR)$(PREFIX)/$(BINDIR)/$(FINBINARY)
 
 uninstall:
	$(RM) -f $(DESTDIR)$(PREFIX)/$(BINDIR)/$(FINBINARY)
	$(RM) -f $(DESTDIR)$(PREFIX)/$(BINDIR)/$(LIST)
 
 clean:
	$(RM) -r $(BUILDDIR)
 
 test: $(BINARY)
	./$(BUILDDIR)/$(BINARY)

 debug:
	$(CC) $(DFLAGS) $(MANDIR)
 
 .PHONY: all clean install test debug