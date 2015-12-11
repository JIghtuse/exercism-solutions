#[derive(Debug,PartialEq)]
pub struct RibonucleicAcid<'a> {
    strand: &'a str,
}

impl<'a> RibonucleicAcid<'a> {
    pub fn new(strand: &str) -> RibonucleicAcid {
        RibonucleicAcid { strand: strand }
    }
    pub fn as_ref(&self) -> &str {
        self.strand
    }
}

#[derive(Debug,PartialEq)]
pub struct DeoxyribonucleicAcid<'a> {
    strand: &'a str,
}

impl<'a> DeoxyribonucleicAcid<'a> {
    pub fn new(strand: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { strand: strand }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid { strand: self.strand }
    }
}
