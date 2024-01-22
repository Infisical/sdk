namespace Infisical.Sdk.Samples;


using Infisical.Sdk;



internal class Program
{
    private static void Main(string[] args)
    {

        ClientSettings settings = new ClientSettings
        {
            ClientId = "Test"
        };


        using var infisicalClient = new InfisicalClient(settings);

        var key = infisicalClient.CreateSymmetricKey();

        Console.WriteLine($"Key: {key}");


        EncryptSymmetricOptions options = new EncryptSymmetricOptions
        {
            Key = key,
            Plaintext = "Hello World!"
        };

        var encrypted = infisicalClient.EncryptSymmetric(options);

        Console.WriteLine($"Encrypted: {encrypted.Ciphertext}");



        DecryptSymmetricOptions decryptOptions = new DecryptSymmetricOptions
        {
            Key = key,
            Ciphertext = encrypted.Ciphertext,
            Iv = encrypted.Iv,
            Tag = encrypted.Tag
        };

        var decrypted = infisicalClient.DecryptSymmetric(decryptOptions);

        Console.WriteLine($"Decrypted: {decrypted}");




    }
}