use std::mem;

#[derive(Debug,PartialEq)]
enum DnaNucleotide {
    G = 71,
    C = 67,
    T = 84,
    A = 65,
}

#[derive(Debug,PartialEq)]
enum RnaNucleotide {
    G = 71,
    C = 67,
    U = 85,
    A = 65,
}

#[derive(Debug,PartialEq)]
pub struct RibonucleicAcid {
    strand: Vec<RnaNucleotide>,
}

impl From<String> for RibonucleicAcid {
    fn from(s: String) -> RibonucleicAcid {
        RibonucleicAcid { strand: unsafe { mem::transmute(s.into_bytes()) } }
    }
}

impl Into<String> for RibonucleicAcid {
    fn into(self) -> String {
        String::from_utf8(unsafe { mem::transmute(self.strand) }).unwrap()
    }
}

impl AsRef<str> for RibonucleicAcid {
    fn as_ref(&self) -> &str {
        unsafe { mem::transmute(&self.strand[..]) }
    }
}

impl RibonucleicAcid {
    pub fn new<S>(strand: S) -> RibonucleicAcid
        where S: Into<String>
    {
        RibonucleicAcid::from(strand.into())
    }
}

#[derive(Debug,PartialEq)]
pub struct DeoxyribonucleicAcid {
    strand: Vec<DnaNucleotide>,
}

impl From<String> for DeoxyribonucleicAcid {
    fn from(s: String) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { strand: unsafe { mem::transmute(s.into_bytes()) } }
    }
}

impl Into<String> for DeoxyribonucleicAcid {
    fn into(self) -> String {
        String::from_utf8(unsafe { mem::transmute(self.strand) }).unwrap()
    }
}

impl AsRef<str> for DeoxyribonucleicAcid {
    fn as_ref(&self) -> &str {
        unsafe { mem::transmute(&self.strand[..]) }
    }
}

impl DeoxyribonucleicAcid {
    pub fn new<S>(strand: S) -> DeoxyribonucleicAcid
        where S: Into<String>
    {
        DeoxyribonucleicAcid::from(strand.into())
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let s = self.as_ref().to_owned();
        RibonucleicAcid::from(s)
    }
}
