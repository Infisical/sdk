package com.infisical.sdk;

import com.infisical.sdk.schema.*;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.sun.jna.Native;
import com.sun.jna.Pointer;

import java.util.function.Function;

public class InfisicalClient implements AutoCloseable {

    private final Pointer client;
    private final InfisicalLibrary library;
    private final CommandRunner commandRunner;
    private boolean isClientOpen;

    public InfisicalClient(ClientSettings settings) {

        library = Native.load("infisical_c", InfisicalLibrary.class);

        try {
            client = library.init(Converter.ClientSettingsToJsonString(settings));
        } catch (JsonProcessingException e) {
            throw new RuntimeException(e);
        }

        commandRunner = new CommandRunner(library, client);
        isClientOpen = true;
    }

    public GetSecretResponseSecret getSecret(GetSecretOptions options) {
        Command command = new Command();
        command.setGetSecret(options);

        ResponseForGetSecretResponse response = commandRunner.runCommand(command,
                InfisicalClient.throwingFunctionWrapper(Converter::ResponseForGetSecretResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData().getSecret();
    }

    public ListSecretsResponse listSecrets(ListSecretsOptions options) {
        Command command = new Command();
        command.setListSecrets(options);

        ResponseForListSecretsResponse response = commandRunner.runCommand(command,
                InfisicalClient.throwingFunctionWrapper(Converter::ResponseForListSecretsResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData();
    }

    public CreateSecretResponse createSecret(CreateSecretOptions options) {
        Command command = new Command();
        command.setCreateSecret(options);

        ResponseForCreateSecretResponse response = commandRunner.runCommand(command,
                InfisicalClient.throwingFunctionWrapper(Converter::ResponseForCreateSecretResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData();
    }

    public UpdateSecretResponse updateSecret(UpdateSecretOptions options) {
        Command command = new Command();
        command.setUpdateSecret(options);

        ResponseForUpdateSecretResponse response = commandRunner.runCommand(command,
                InfisicalClient.throwingFunctionWrapper(Converter::ResponseForUpdateSecretResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData();
    }

    public DeleteSecretResponse deleteSecret(DeleteSecretOptions options) {
        Command command = new Command();
        command.setDeleteSecret(options);

        ResponseForDeleteSecretResponse response = commandRunner.runCommand(command,
                InfisicalClient.throwingFunctionWrapper(Converter::ResponseForDeleteSecretResponseFromJsonString));

        errorCheck(response.getSuccess(), response.getErrorMessage());
        return response.getData();
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
