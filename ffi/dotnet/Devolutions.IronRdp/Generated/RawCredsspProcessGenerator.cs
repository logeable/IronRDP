// <auto-generated/> by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using Devolutions.IronRdp.Diplomat;
#pragma warning restore 0105

namespace Devolutions.IronRdp.Raw;

#nullable enable

[StructLayout(LayoutKind.Sequential)]
public partial struct CredsspProcessGenerator
{
    private const string NativeLib = "DevolutionsIronRdp";

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "CredsspProcessGenerator_start", ExactSpelling = true)]
    public static unsafe extern CredsspNetworkFfiResultBoxGeneratorStateBoxIronRdpError Start(CredsspProcessGenerator* self);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "CredsspProcessGenerator_resume", ExactSpelling = true)]
    public static unsafe extern CredsspNetworkFfiResultBoxGeneratorStateBoxIronRdpError Resume(CredsspProcessGenerator* self, byte* response, nuint responseSz);

    [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "CredsspProcessGenerator_destroy", ExactSpelling = true)]
    public static unsafe extern void Destroy(CredsspProcessGenerator* self);
}