package cinterface

import (
	"fmt"
	"unsafe"
)

/*
#cgo LDFLAGS: -linfisical_c
#cgo linux,amd64 LDFLAGS: -L ./lib/linux-x64
#cgo linux,arm64 LDFLAGS: -L ./lib/linux-arm64
#cgo darwin,arm64 LDFLAGS: -L ./lib/darwin-arm64 -framework Security -framework SystemConfiguration
#cgo darwin,amd64 LDFLAGS: -L ./lib/darwin-x64 -framework Security -framework SystemConfiguration
#cgo windows,amd64 LDFLAGS: -L ./lib/windows-x64 -linfisical_c -ladvapi32 -lbcrypt -lcrypt32 -lcryptnet -lkernel32 -lncrypt -lntdll -luserenv -lws2_32 -lmsvcrt
#include <stdlib.h>
typedef void* ClientPtr;
extern char* run_command(const char *command, ClientPtr client);
extern ClientPtr init(const char *clientSettings);
extern void free_mem(ClientPtr client);
*/
import "C"

type ClientPointer struct {
	Pointer C.ClientPtr
}

type InfisicalLibrary interface {
	Init(clientSettings string) (ClientPointer, error)
	FreeMem(client ClientPointer)
	RunCommand(command string, client ClientPointer) (string, error)
}

type InfisicalLibraryImpl struct{}

func NewInfisicalLibrary() InfisicalLibrary {
	return &InfisicalLibraryImpl{}
}

func (b *InfisicalLibraryImpl) Init(clientSettings string) (ClientPointer, error) {
	ptr := C.init(C.CString(clientSettings))
	if ptr == nil {
		return ClientPointer{}, fmt.Errorf("initialization failed")
	}
	return ClientPointer{Pointer: ptr}, nil
}

func (b *InfisicalLibraryImpl) FreeMem(client ClientPointer) {
	C.free_mem(client.Pointer)
}

func (b *InfisicalLibraryImpl) RunCommand(command string, client ClientPointer) (string, error) {
	cstr := C.run_command(C.CString(command), client.Pointer)
	if cstr == nil {
		return "", fmt.Errorf("run command failed")
	}
	defer C.free(unsafe.Pointer(cstr))
	return C.GoString(cstr), nil
}
