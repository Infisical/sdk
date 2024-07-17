package com.infisical.sdk;

import com.infisical.sdk.schema.*;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.sun.jna.Native;
import com.sun.jna.Pointer;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.function.Function;

public class InfisicalClient implements AutoCloseable {

    private final Pointer client;
    private final InfisicalLibrary library;
    private final CommandRunner commandRunner;
    private boolean isClientOpen;

    public InfisicalClient(ClientSettings settings) {

        settings.setUserAgent("infisical-java-sdk");

        String libraryName = "infisical_c";
        String arch = System.getProperty("os.arch");
        String os = System.getProperty("os.name").toLowerCase();

        if (os.contains("linux") && arch.equals("aarch64")) {
          if (isMusl()) {
              libraryName = "/linux-aarch64-musl/libinfisical_c";
          } else {
              libraryName = "/linux-aarch64-gnu/libinfisical_c";
          }
        }

        library = Native.load(libraryName, InfisicalLibrary.class);

        try {
            client = library.init(Converter.ClientSettingsToJsonString(settings));
        } catch (JsonProcessingException e) {
            throw new RuntimeException(e);
        }

        commandRunner = new CommandRunner(library, client);
        isClientOpen = true;
    }

    @SuppressWarnings("deprecation")
    private static boolean isMusl() {
        try {
            Process process = Runtime.getRuntime().exec("ldd --version");
            BufferedReader reader = new BufferedReader(new InputStreamReader(process.getErrorStream()));
            String line = reader.readLine();
            return line != null && line.contains("musl");
        } catch (IOException e) {
            // If we can't determine, assume it's not musl
            return false;
        }
    }

    public GetSecretResponseSecret getSecret(GetSecretOptions options) {
        Command command = new Command();
        command.setGetSecret(options);

        ResponseForGetSecretResponse response = commandRunner.runCommand(command,
                InfisicalClient.throwingFunctionWrapper(Converter::ResponseForGetSecretResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());

        return response.getData().getSecret();
    }

    public SecretElement[] listSecrets(ListSecretsOptions options) {
        Command command = new Command();
        command.setListSecrets(options);

        ResponseForListSecretsResponse response = commandRunner.runCommand(command,
                InfisicalClient.throwingFunctionWrapper(Converter::ResponseForListSecretsResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());

        return response.getData().getSecrets();
    }

    public CreateSecretResponseSecret createSecret(CreateSecretOptions options) {
        Command command = new Command();
        command.setCreateSecret(options);

        ResponseForCreateSecretResponse response = commandRunner.runCommand(command,
                InfisicalClient.throwingFunctionWrapper(Converter::ResponseForCreateSecretResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData().getSecret();
    }

    public UpdateSecretResponseSecret updateSecret(UpdateSecretOptions options) {
        Command command = new Command();
        command.setUpdateSecret(options);

        ResponseForUpdateSecretResponse response = commandRunner.runCommand(command,
                InfisicalClient.throwingFunctionWrapper(Converter::ResponseForUpdateSecretResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData().getSecret();
    }

    public DeleteSecretResponseSecret deleteSecret(DeleteSecretOptions options) {
        Command command = new Command();
        command.setDeleteSecret(options);

        ResponseForDeleteSecretResponse response = commandRunner.runCommand(command,
                InfisicalClient.throwingFunctionWrapper(Converter::ResponseForDeleteSecretResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData().getSecret();
    }

    public String createSymmetricKey() {
        ArbitraryOptions options = new ArbitraryOptions();
        options.setData("");

        Command command = new Command();
        command.setCreateSymmetricKey(options);

        ResponseForCreateSymmetricKeyResponse response = commandRunner.runCommand(command,
                InfisicalClient
                        .throwingFunctionWrapper(Converter::ResponseForCreateSymmetricKeyResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData().getKey();
    }

    public EncryptSymmetricResponse encryptSymmetric(EncryptSymmetricOptions options) {
        Command command = new Command();
        command.setEncryptSymmetric(options);

        ResponseForEncryptSymmetricResponse response = commandRunner.runCommand(command,
                InfisicalClient
                        .throwingFunctionWrapper(Converter::ResponseForEncryptSymmetricResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData();
    }

    public String decryptSymmetric(DecryptSymmetricOptions options) {
        Command command = new Command();
        command.setDecryptSymmetric(options);

        ResponseForDecryptSymmetricResponse response = commandRunner.runCommand(command,
                InfisicalClient
                        .throwingFunctionWrapper(Converter::ResponseForDecryptSymmetricResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData().getDecrypted();
    }

    private void errorCheck(boolean success, String errorMessage) {
        if (!success) {
            if (errorMessage.length() > 0) {
                throw new RuntimeException(errorMessage);
            }

            throw new RuntimeException("Command failed to execute with no error code");
        }
    }

    static <T, R> Function<T, R> throwingFunctionWrapper(ThrowingFunction<T, R, Exception> throwingFunction) {

        return i -> {
            try {
                return throwingFunction.accept(i);
            } catch (Exception ex) {
                throw new RuntimeException("Response deserialization failed");
            }
        };
    }

    @Override
    public void close() {
        if (isClientOpen) {
            library.free_mem(client);
            isClientOpen = false;
        }
    }

}
