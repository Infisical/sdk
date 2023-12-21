package com.infisical.sdk;

import com.sun.jna.Library;
import com.sun.jna.Pointer;

public interface InfisicalLibrary extends Library {

    Pointer init(String clientSettings);

    void free_mem(Pointer client);

    String run_command(String command, Pointer client);
}
