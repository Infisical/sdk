namespace Infisical.Sdk;

public sealed class InfisicalClient : IDisposable
{
    private readonly CommandRunner _commandRunner;
    private readonly InfisicalHandle _handle;

    public InfisicalClient(ClientSettings settings)
    {
        settings.UserAgent = "Infisical.Sdk";

        _handle = InfisicalLibrary.Init(settings.ToJson());
        _commandRunner = new CommandRunner(_handle);
    }

    public GetSecretResponseSecret GetSecret(GetSecretOptions options)
    {

        var cmd = new Command
        {
            GetSecret = options
        };

        var result = _commandRunner.RunCommand<ResponseForGetSecretResponse>(cmd);

        if (result is { Success: true })
        {
            return result.Data.Secret;
        }

        throw new InfisicalException(result == null ? "Unknown error" : result.ErrorMessage);
    }

    public SecretElement[] ListSecrets(ListSecretsOptions options)
    {
        var cmd = new Command
        {
            ListSecrets = options
        };

        var result = _commandRunner.RunCommand<ResponseForListSecretsResponse>(cmd);

        if (result is { Success: true })
        {
            return result.Data.Secrets;
        }

        throw new InfisicalException(result == null ? "Unknown error" : result.ErrorMessage);
    }

    public CreateSecretResponseSecret CreateSecret(CreateSecretOptions options)
    {
        var cmd = new Command
        {
            CreateSecret = options
        };

        var result = _commandRunner.RunCommand<ResponseForCreateSecretResponse>(cmd);

        if (result is { Success: true })
        {
            return result.Data.Secret;
        }

        throw new InfisicalException(result == null ? "Unknown error" : result.ErrorMessage);
    }

    public UpdateSecretResponseSecret UpdateSecret(UpdateSecretOptions options)
    {
        var cmd = new Command
        {
            UpdateSecret = options
        };

        var result = _commandRunner.RunCommand<ResponseForUpdateSecretResponse>(cmd);

        if (result is { Success: true })
        {
            return result.Data.Secret;
        }

        throw new InfisicalException(result == null ? "Unknown error" : result.ErrorMessage);
    }

    public DeleteSecretResponseSecret DeleteSecret(DeleteSecretOptions options)
    {
        var cmd = new Command
        {
            DeleteSecret = options
        };

        var result = _commandRunner.RunCommand<ResponseForDeleteSecretResponse>(cmd);

        if (result is { Success: true })
        {
            return result.Data.Secret;
        }

        throw new InfisicalException(result == null ? "Unknown error" : result.ErrorMessage);
    }

    public string createSymmetricKey()
    {

        var cmd = new Command
        {
            CreateSymmetricKey = new ArbitraryOptions
            {
                Data = ""
            }
        };

        var result = _commandRunner.RunCommand<ResponseForCreateSymmetricKeyResponse>(cmd);

        if (result is { Success: true })
        {
            return result.Data.Key;
        }

        throw new InfisicalException(result == null ? "Unknown error" : result.ErrorMessage);
    }


    public EncryptSymmetricResponse encryptSymmetric(EncryptSymmetricOptions options)
    {
        var cmd = new Command
        {
            EncryptSymmetric = options
        };

        var result = _commandRunner.RunCommand<ResponseForEncryptSymmetricResponse>(cmd);

        if (result is { Success: true })
        {
            return result.Data;
        }

        throw new InfisicalException(result == null ? "Unknown error" : result.ErrorMessage);
    }

    public string decryptSymmetric(DecryptSymmetricOptions options)
    {
        var cmd = new Command
        {
            DecryptSymmetric = options
        };

        var result = _commandRunner.RunCommand<ResponseForDecryptSymmetricResponse>(cmd);

        if (result is { Success: true })
        {
            return result.Data.Decrypted;
        }

        throw new InfisicalException(result == null ? "Unknown error" : result.ErrorMessage);
    }



    public void Dispose() => _handle.Dispose();
}
