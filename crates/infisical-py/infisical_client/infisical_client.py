import json
from typing import Any, List
from .schemas import ClientSettings, Command, SecretElement
from .schemas import GetSecretOptions, ResponseForGetSecretResponse
from .schemas import ListSecretsOptions, ResponseForListSecretsResponse
from .schemas import UpdateSecretOptions, ResponseForUpdateSecretResponse
from .schemas import DeleteSecretOptions, ResponseForDeleteSecretResponse
from .schemas import CreateSecretOptions, ResponseForCreateSecretResponse
import infisical_py

class InfisicalClient:
    def __init__(self, settings: ClientSettings = None):

        if settings is None:
            self.inner = infisical_py.InfisicalClient(None)
        else:
            settings_json = json.dumps(settings.to_dict())

            self.inner = infisical_py.InfisicalClient(settings_json)

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

        return ResponseForListSecretsResponse.from_dict(result).data.secrets
    
    def updateSecret(self, options: UpdateSecretOptions) -> SecretElement:
        result = self._run_command(Command(update_secret=options))

        return ResponseForUpdateSecretResponse.from_dict(result).data.secret
    
    def deleteSecret(self, options: DeleteSecretOptions) -> SecretElement:
        result = self._run_command(Command(delete_secret=options))

        return ResponseForDeleteSecretResponse.from_dict(result).data.secret
    
    def createSecret(self, options: CreateSecretOptions) -> SecretElement:
        result = self._run_command(Command(create_secret=options))

        return ResponseForCreateSecretResponse.from_dict(result).data.secret