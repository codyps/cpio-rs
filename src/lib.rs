use std::{io,fs};

/*!
 * Each cpio archive consists of a concatenation of one or more member `file`s. Each `file` consists
 * of a header (`CpioFileHeader`) optionally followed by contents indicated by the header.
 *
 * End of archive is marked by a special `file` named `'TRAILER!!'`
 */

/**
 * Write a cpio archive from scratch (or append to one with the trailer stripped).
 */
struct CpioWriter<W: io::Writer> {
    w: W 
}

impl CpioWriter {
    pub fn from<W: io::Writer>(w: W) {
        CpioWriter { w: w } 
    }
}

/**
 * Header for a cpio archive. Can be stored either as ASCII or binary
 */
struct CpioFileHeader {
    pub dev:  u32,
    pub ino:  u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub nlink: u32,

    /* only present in 'new ascii' formats */
    pub dev_major: u32,
    pub dev_minor: u32,

    /* ascii & portable-bin only has 1 rdev */
    /* portable-bin limited to u16 */
    pub rdev_major: u32,
    pub rdev_minor: u32,

    pub mtime: u32,
    pub namesize: u32,
    pub filesize: u32,

    pub check: u32,
}

impl CpioFileHeader {
    /**
     *  - can be LE or BE formatted
     *  - 2 & 4 byte binary values
     */
    pub fn write_old_binary<W: io::Writer>(w: W) -> io::Result<()> {

    }

    /**
     * aka SUSv2 standard
     *
     *  - uses 6-byte & 11-byte octal fields
     */
    pub fn write_portable_ascii<W: io::Writer>(w: W) -> io::Result<()> {
        write!(w, "{:06o}", w.dev)
    }

    /**
     *
     * Compared to `write_portable_ascii`
     *
     *  - uses 8-byte hexadecimal fields
     *  - splits dev & rdev into major & minor (doubling their effective size)
     *  
     *
     */
    pub fn write_new_ascii<W: io::Writer>(w: W) -> io::Result<()> {

    }

    /**
     * Also refered to as 'sv4crc'
     *
     * Compared to `write_new_ascii`:
     *
     *  - Uses a basic checksum (summing bytes) for each file in the archive. Headers are not
     *    checksumed. Despite the name, this does _not_ use a CRC.
     */
    pub fn write_new_ascii_crc<W: io::Writer>(w: W) -> io::Result<()> {

    }

    /**
     * read a header
     */
    pub fn read<R: io::Reader>(r: R) -> io::Result<()> {

    }
}

/* header parsing */


#[test]
fn it_works() {
}
