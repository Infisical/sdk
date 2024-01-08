using System.Runtime.InteropServices;

namespace Infisical.Sdk;

internal static class InfisicalLibrary
{
    [DllImport("infisical_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern InfisicalHandle init(string settings);

    [DllImport("infisical_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern void free_mem(IntPtr handle);

    [DllImport("infisical_c", CallingConvention = CallingConvention.Cdecl)]
    private static extern string run_command(string json, InfisicalHandle handle);

    internal static InfisicalHandle Init(string settings) => init(settings);

    internal static void FreeMemory(IntPtr handle) => free_mem(handle);

    internal static string RunCommand(string json, InfisicalHandle handle) => run_command(json, handle);
}
