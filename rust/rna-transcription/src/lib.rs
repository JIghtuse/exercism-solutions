#[derive(Debug,PartialEq)]
pub struct RibonucleicAcid {
    strand: String,
}

impl RibonucleicAcid {
    pub fn new(strand: &str) -> RibonucleicAcid {
        RibonucleicAcid { strand: strand.to_owned() }
    }
    pub fn as_ref(&self) -> &str {
        self.strand.as_ref()
    }
}

#[derive(Debug,PartialEq)]
pub struct DeoxyribonucleicAcid {
    strand: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(strand: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { strand: strand.to_owned() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid {
            strand: self.strand
                        .replace("G", "C")
                        .replace("C", "G")
                        .replace("T", "A")
                        .replace("A", "U")
        }
    }
}
