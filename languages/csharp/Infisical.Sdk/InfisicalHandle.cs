using Microsoft.Win32.SafeHandles;

namespace Infisical.Sdk;

internal class InfisicalHandle : SafeHandleZeroOrMinusOneIsInvalid
{
    public InfisicalHandle() : base(true)
    {
        SetHandle(handle);
    }

    protected override bool ReleaseHandle()
    {
        InfisicalLibrary.FreeMemory(handle);
        return true;
    }
}
