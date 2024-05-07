// <auto-generated/> by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using Devolutions.IronRdp.Diplomat;
#pragma warning restore 0105

namespace Devolutions.IronRdp;

#nullable enable

public partial class DecodedImage: IDisposable
{
    private unsafe Raw.DecodedImage* _inner;

    public BytesSlice Data
    {
        get
        {
            return GetData();
        }
    }

    public ushort Height
    {
        get
        {
            return GetHeight();
        }
    }

    public ushort Width
    {
        get
        {
            return GetWidth();
        }
    }

    /// <summary>
    /// Creates a managed <c>DecodedImage</c> from a raw handle.
    /// </summary>
    /// <remarks>
    /// Safety: you should not build two managed objects using the same raw handle (may causes use-after-free and double-free).
    /// <br/>
    /// This constructor assumes the raw struct is allocated on Rust side.
    /// If implemented, the custom Drop implementation on Rust side WILL run on destruction.
    /// </remarks>
    public unsafe DecodedImage(Raw.DecodedImage* handle)
    {
        _inner = handle;
    }

    /// <returns>
    /// A <c>DecodedImage</c> allocated on Rust side.
    /// </returns>
    public static DecodedImage New(PixelFormat pixelFormat, ushort width, ushort height)
    {
        unsafe
        {
            Raw.PixelFormat pixelFormatRaw;
            pixelFormatRaw = (Raw.PixelFormat)pixelFormat;
            Raw.DecodedImage* retVal = Raw.DecodedImage.New(pixelFormatRaw, width, height);
            return new DecodedImage(retVal);
        }
    }

    /// <returns>
    /// A <c>BytesSlice</c> allocated on Rust side.
    /// </returns>
    public BytesSlice GetData()
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("DecodedImage");
            }
            Raw.BytesSlice* retVal = Raw.DecodedImage.GetData(_inner);
            return new BytesSlice(retVal);
        }
    }

    public ushort GetWidth()
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("DecodedImage");
            }
            ushort retVal = Raw.DecodedImage.GetWidth(_inner);
            return retVal;
        }
    }

    public ushort GetHeight()
    {
        unsafe
        {
            if (_inner == null)
            {
                throw new ObjectDisposedException("DecodedImage");
            }
            ushort retVal = Raw.DecodedImage.GetHeight(_inner);
            return retVal;
        }
    }

    /// <summary>
    /// Returns the underlying raw handle.
    /// </summary>
    public unsafe Raw.DecodedImage* AsFFI()
    {
        return _inner;
    }

    /// <summary>
    /// Destroys the underlying object immediately.
    /// </summary>
    public void Dispose()
    {
        unsafe
        {
            if (_inner == null)
            {
                return;
            }

            Raw.DecodedImage.Destroy(_inner);
            _inner = null;

            GC.SuppressFinalize(this);
        }
    }

    ~DecodedImage()
    {
        Dispose();
    }
}