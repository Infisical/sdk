import * as rust from "../../binding";
import { LogLevel } from "../../binding";
import { ClientSettings, Convert } from "./schemas";

import type { GetSecretOptions, GetSecretResponse } from "./schemas";
import type { ListSecretsOptions, ListSecretsResponse } from "./schemas";
import type { UpdateSecretOptions, UpdateSecretResponse } from "./schemas";
import type { CreateSecretOptions, CreateSecretResponse } from "./schemas";
import type { DeleteSecretOptions, DeleteSecretResponse } from "./schemas";

import type { DecryptSymmetricOptions } from "./schemas";
import type { EncryptSymmetricOptions, EncryptSymmetricResponse } from "./schemas";

export class InfisicalClient {
    #client: rust.InfisicalClient;

    constructor(settings: ClientSettings & { logLevel?: LogLevel }) {
        const settingsJson = settings == null ? null : Convert.clientSettingsToJson(settings);
        this.#client = new rust.InfisicalClient(settingsJson, settings.logLevel ?? LogLevel.Error);
    }

    async getSecret(options: GetSecretOptions): Promise<GetSecretResponse["secret"]> {
        const command = await this.#client.runCommand(
            Convert.commandToJson({
                getSecret: options
            })
        );
        const response = Convert.toResponseForGetSecretResponse(command);

        if (!response.success || response.data == null) {
            throw new Error(response.errorMessage ?? "Something went wrong");
        }

        return response.data.secret;
    }

    async listSecrets(options: ListSecretsOptions): Promise<ListSecretsResponse["secrets"]> {
        const command = await this.#client.runCommand(
            Convert.commandToJson({
                listSecrets: options
            })
        );
        const response = Convert.toResponseForListSecretsResponse(command);

        if (!response.success || response.data == null) {
            throw new Error(response.errorMessage ?? "Something went wrong");
        }

        return response.data.secrets;
    }

    async updateSecret(options: UpdateSecretOptions): Promise<UpdateSecretResponse["secret"]> {
        const command = await this.#client.runCommand(
            Convert.commandToJson({
                updateSecret: options
            })
        );
        const response = Convert.toResponseForUpdateSecretResponse(command);

        if (!response.success || response.data == null) {
            throw new Error(response.errorMessage ?? "Something went wrong");
        }

        return response.data.secret;
    }

    async deleteSecret(options: DeleteSecretOptions): Promise<DeleteSecretResponse["secret"]> {
        const command = await this.#client.runCommand(
            Convert.commandToJson({
                deleteSecret: options
            })
        );
        const response = Convert.toResponseForDeleteSecretResponse(command);

        if (!response.success || response.data == null) {
            throw new Error(response.errorMessage ?? "Something went wrong");
        }

        return response.data.secret;
    }

    async createSecret(options: CreateSecretOptions): Promise<CreateSecretResponse["secret"]> {
        const command = await this.#client.runCommand(
            Convert.commandToJson({
                createSecret: options
            })
        );
        const response = Convert.toResponseForCreateSecretResponse(command);

        if (!response.success || response.data == null) {
            throw new Error(response.errorMessage ?? "Something went wrong");
        }

        return response.data.secret;
    }

    // Has to be a promise because our client is async
    async createSymmetricKey(): Promise<string> {
        const command = await this.#client.runCommand(
            Convert.commandToJson({
                createSymmetricKey: {
                    data: ""
                }
            })
        );
        const response = Convert.toResponseForCreateSymmetricKeyResponse(command);

        if (!response.success || response.data == null) {
            throw new Error(response.errorMessage ?? "Something went wrong");
        }

        return response.data.key;
    }

    async encryptSymmetric(options: EncryptSymmetricOptions): Promise<EncryptSymmetricResponse> {
        const command = await this.#client.runCommand(
            Convert.commandToJson({
                encryptSymmetric: options
            })
        );
        const response = Convert.toResponseForEncryptSymmetricResponse(command);

        if (!response.success || response.data == null) {
            throw new Error(response.errorMessage ?? "Something went wrong");
        }

        return response.data;
    }

    async decryptSymmetric(options: DecryptSymmetricOptions): Promise<string> {
        const command = await this.#client.runCommand(
            Convert.commandToJson({
                decryptSymmetric: options
            })
        );
        const response = Convert.toResponseForDecryptSymmetricResponse(command);

        if (!response.success || response.data == null) {
            throw new Error(response.errorMessage ?? "Something went wrong");
        }

        return response.data.decrypted;
    }
}

export { LogLevel };
