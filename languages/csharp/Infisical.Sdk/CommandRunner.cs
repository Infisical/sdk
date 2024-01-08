using System.Text.Json;

namespace Infisical.Sdk;

internal class CommandRunner
{
    private readonly InfisicalHandle _handle;

    internal CommandRunner(InfisicalHandle handle)
    {
        _handle = handle;
    }

    internal T? RunCommand<T>(Command command)
    {
        var req = JsonSerializer.Serialize(command, Converter.Settings);
        var result = InfisicalLibrary.RunCommand(req, _handle);
        return JsonSerializer.Deserialize<T>(result, Converter.Settings);
    }
}
