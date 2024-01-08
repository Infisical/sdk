namespace Infisical.Sdk;

public class InfisicalException : Exception
{
    public InfisicalException(string message) : base(message)
    {
    }

    public InfisicalException(string message, Exception innerException)
        : base(message, innerException)
    {
    }
}
