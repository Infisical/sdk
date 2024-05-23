import json
from typing import Any, List
from .schemas import ClientSettings, Command, SecretElement
from .schemas import GetSecretOptions, ResponseForGetSecretResponse
from .schemas import ListSecretsOptions, ResponseForListSecretsResponse
from .schemas import UpdateSecretOptions, ResponseForUpdateSecretResponse
from .schemas import DeleteSecretOptions, ResponseForDeleteSecretResponse
from .schemas import CreateSecretOptions, ResponseForCreateSecretResponse

from .schemas import EncryptSymmetricOptions, EncryptSymmetricResponse, ResponseForEncryptSymmetricResponse
from .schemas import DecryptSymmetricOptions, ResponseForDecryptSymmetricResponse

from .schemas import ArbitraryOptions, ResponseForCreateSymmetricKeyResponse

from .infisical_py import InfisicalClient as RustInfisicalClient
import os

class InfisicalClient:
    def __init__(self, settings: ClientSettings = None, debug: bool = False):

        if settings is None:
            self.inner = RustInfisicalClient(settings, debug)
        else:

            settings.user_agent = "infisical-python-sdk"

            settings_json = json.dumps(settings.to_dict())

            self.inner = RustInfisicalClient(settings_json, debug)

    def _run_command(self, command: Command) -> Any:
        response_json = self.inner.run_command(json.dumps(command.to_dict()))
        response = json.loads(response_json)

        if response["success"] == False:
            raise Exception(response["errorMessage"])
        
        return response
    

    def getSecret(self, options: GetSecretOptions) -> SecretElement:
        result = self._run_command(Command(get_secret=options))

        return ResponseForGetSecretResponse.from_dict(result).data.secret
    
    def listSecrets(self, options: ListSecretsOptions) -> List[SecretElement]:
        result = self._run_command(Command(list_secrets=options))

        secrets = ResponseForListSecretsResponse.from_dict(result).data.secrets

        # Setting the env in Rust is not enough for Python apparently, so we have to do this as well.
        if options.attach_to_process_env:
            for secret in secrets:
              # we need to check if the env variable is already set, if it is we don't want to overwrite it!
              if os.environ.get(secret.secret_key) is None:
                  os.environ[secret.secret_key] = secret.secret_value

        return secrets
    
    def updateSecret(self, options: UpdateSecretOptions) -> SecretElement:
        result = self._run_command(Command(update_secret=options))

        return ResponseForUpdateSecretResponse.from_dict(result).data.secret
    
    def deleteSecret(self, options: DeleteSecretOptions) -> SecretElement:
        result = self._run_command(Command(delete_secret=options))

        return ResponseForDeleteSecretResponse.from_dict(result).data.secret
    
    def createSecret(self, options: CreateSecretOptions) -> SecretElement:
        result = self._run_command(Command(create_secret=options))

        return ResponseForCreateSecretResponse.from_dict(result).data.secret
    
    def createSymmetricKey(self) -> str:

        arbitraryOptions = ArbitraryOptions(data="")

        result = self._run_command(Command(create_symmetric_key=arbitraryOptions))

        return ResponseForCreateSymmetricKeyResponse.from_dict(result).data.key
    
    def encryptSymmetric(self, options: EncryptSymmetricOptions) -> EncryptSymmetricResponse:
        result = self._run_command(Command(encrypt_symmetric=options))

        return ResponseForEncryptSymmetricResponse.from_dict(result).data
    
    def decryptSymmetric(self, options: DecryptSymmetricOptions) -> str:
        result = self._run_command(Command(decrypt_symmetric=options))

        return ResponseForDecryptSymmetricResponse.from_dict(result).data.decrypted