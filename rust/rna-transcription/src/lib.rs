use std::mem;

#[derive(Debug,PartialEq)]
enum DnaNucleotide {
    A = 65,
    C = 67,
    G = 71,
    T = 84,
}

#[derive(Debug,PartialEq)]
enum RnaNucleotide {
    A = 65,
    C = 67,
    G = 71,
    U = 85,
}

#[derive(Debug,PartialEq)]
pub struct RibonucleicAcid {
    strand: Vec<RnaNucleotide>,
}

impl From<String> for RibonucleicAcid {
    fn from(s: String) -> RibonucleicAcid {
        for (i, c) in s.bytes().enumerate() {
            match c {
                65 | 67 | 71 | 85 => {}
                _ => panic!("Incorrect nucleotide at position {}", i),
            }
        }
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
        unsafe { ::std::str::from_utf8_unchecked(mem::transmute(&self.strand[..])) }
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
        for (i, c) in s.bytes().enumerate() {
            match c {
                65 | 67 | 71 | 84 => {}
                _ => panic!("Incorrect nucleotide at position {}", i),
            }
        }
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
        unsafe { ::std::str::from_utf8_unchecked(mem::transmute(&self.strand[..])) }
    }
}

impl DeoxyribonucleicAcid {
    pub fn new<S>(strand: S) -> DeoxyribonucleicAcid
        where S: Into<String>
    {
        DeoxyribonucleicAcid::from(strand.into())
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let rna_strand = self.strand
                             .iter()
                             .map(|ref n| {
                                 match **n {
                                     DnaNucleotide::G => RnaNucleotide::C,
                                     DnaNucleotide::C => RnaNucleotide::G,
                                     DnaNucleotide::T => RnaNucleotide::A,
                                     DnaNucleotide::A => RnaNucleotide::U,
                                 }
                             })
                             .collect();
        RibonucleicAcid { strand: rna_strand }
    }
}
